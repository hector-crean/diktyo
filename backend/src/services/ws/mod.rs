use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    headers::UserAgent,
    response::IntoResponse,
    TypedHeader,
};

use serde_json::json;

use std::{net::SocketAddr, ops::ControlFlow, time::Duration};

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

use crate::AppState;
use bibe_models::message::BibeMsg;

pub async fn ws_handler(
    state: State<AppState>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.

    ws.on_upgrade(move |socket| websocket(state, socket, addr))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(State(state): State<AppState>, mut socket: WebSocket, who: SocketAddr) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {}...", who);
    } else {
        println!("Could not send ping {}!", who);
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let join_msg = BibeMsg::SwitchBikeOnline {
        socket_address: who,
    };
    tracing::debug!("{:?}", join_msg);
    let _ = state.tx.send(join_msg);

    // // Spawn a task that will push several messages to the client (does not matter what client does)
    // let mut send_task = tokio::spawn(async move {
    //     while let Ok(msg) = rx.recv().await {
    //         // In any websocket error, break loop.

    //         if sender
    //             .send(Message::Text(json!(msg).to_string()))
    //             .await
    //             .is_err()
    //         {
    //             break;
    //         }
    //     }
    // });

    let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(5));

    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
            _ = heartbeat_interval.tick() => {
                // Sending a ping at every tick of the heartbeat interval.
                // If the sending fails, we break out of the loop, ending the task.
                if sender.send(Message::Ping(Vec::new())).await.is_err() {
                    break;
                }
            }
            Ok(msg) = rx.recv() => {
                let send_msg = sender
                .send(Message::Text(json!(msg).to_string()))
                .await;

                if send_msg.is_err()  {break; }

            },

            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => match serde_json::from_str::<BibeMsg>(&text) {
                    Ok(msg) => {
                        let _ = tx.send(msg);
                    }
                    Err(e) => {
                        // Log deserialization errors to help diagnose issues
                        tracing::error!(
                            "Failed to deserialize message: {:?}. Text was: {:?}",
                            e,
                            text
                        );
                    }
                },
                Ok(Message::Ping(ping_msg)) => {
                    // Optionally, handle Ping messages
                    tracing::debug!("Received Ping message: {:?}", ping_msg);
                }
                Ok(Message::Pong(pong_msg)) => {
                    // Optionally, handle Pong messages
                    tracing::debug!("Received Pong message: {:?}", pong_msg);
                }
                Ok(Message::Close(Some(close_frame))) => {
                    // Handle close messages by possibly acknowledging them before breaking the loop
                    tracing::debug!("Received Close message: {:?}", close_frame);
                    // let _ = sender.send(Message::Close(Some(close_frame))).await;
                    break;
                }
                Ok(Message::Close(None)) => {
                    // Handle close messages with no frame
                    tracing::debug!("Received Close message with no frame");
                    break;
                }
                Ok(Message::Binary(_)) => {
                    // Optionally, handle Binary messages (but in many cases you might just ignore these)
                    tracing::warn!("Received a binary message, which is unsupported");
                }
                Err(e) => {
                    // Log other errors
                    tracing::error!("WebSocket error: {:?}", e);
                    break;
                }
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) =>  tracing::info!("{:?} messages sent to {:?}", a, who),
                Err(a) =>  tracing::info!("Error sending messages {:?}", a)
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) =>  tracing::info!("Received {:?} messages", b),
                Err(b) =>  tracing::info!("Error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }

    // Send "user left" message (similar to "joined" above).
    let exit_msg = BibeMsg::SwitchBikeOffline {
        socket_address: who,
    };
    tracing::debug!("{:?}", exit_msg);
    let _ = state.tx.send(exit_msg);

    // returning from the handler closes the websocket connection
    tracing::info!("Websocket context {} destroyed", who);
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            tracing::info!(">>> {} sent str: {:?}", who, t);
        }
        Message::Binary(d) => {
            tracing::info!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> {} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            tracing::info!(">>> {} sent pong with {:?}", who, v);
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            tracing::info!(">>> {} sent ping with {:?}", who, v);
        }
    }
    ControlFlow::Continue(())
}
