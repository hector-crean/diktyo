pub mod fsm;
pub mod gpio;

use bibe_models::command::Messages;
use futures_util::stream::FuturesUnordered;
use futures_util::{SinkExt, StreamExt};
use std::borrow::Cow;
use std::ops::ControlFlow;
use std::time::Instant;
use tokio::task::JoinError;

use tokio::time::{self, Duration};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::frame::coding::CloseCode,
    tungstenite::protocol::CloseFrame, tungstenite::Message,
};

async fn spawn_client(who: usize) {
    let ws_stream = match connect_async(SERVER).await {
        Ok((stream, response)) => {
            println!("Handshake for client {} has been completed", who);
            // This will be the HTTP response, same as with server this is the last moment we
            // can still access HTTP stuff.
            println!("Server response was {:?}", response);
            stream
        }
        Err(e) => {
            println!("WebSocket handshake for client {who} failed with {e}!");
            return;
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();

    //we can ping the server for start
    sender
        .send(Message::Ping("Hello, Server!".into()))
        .await
        .expect("Can not send!");

    // Create a heartbeat interval. Adjust the duration to your liking.
    let mut heartbeat_interval = time::interval(Duration::from_secs(5));

    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = heartbeat_interval.tick() => {
                    // Sending a ping at every tick of the heartbeat interval.
                    if sender.send(Message::Ping(Vec::new())).await.is_err() {
                        break;
                    }
                }
                // Other send logic goes here, including sending text messages and eventually closing the connection.
                // ...
            }
        }
    });

    //receiver just prints whatever it gets
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
    });

    //wait for either task to finish and kill the other task
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}

/// Function to handle messages we get (with a slight twist that Frame variant is visible
/// since we are working with the underlying tungstenite library directly without axum here).
fn process_message(msg: Message, who: usize) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {} got str: {:?}", who, t);
        }
        Message::Binary(d) => {
            println!(">>> {} got {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} got close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {} somehow got close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {} got pong with {:?}", who, v);
        }
        // Just as with axum server, the underlying tungstenite websocket library
        // will handle Ping for you automagically by replying with Pong and copying the
        // v according to spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {} got ping with {:?}", who, v);
        }

        Message::Frame(_) => {
            unreachable!("This is never supposed to happen")
        }
    }
    ControlFlow::Continue(())
}
