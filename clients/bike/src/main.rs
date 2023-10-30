use bibe_bike::spawn_client;
use tokio::task::JoinError;

const CLIENT_ID: usize = 1;
const WS_ENDPOINT: &str = "ws://127.0.0.1:3000/v1/api/ws";

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    let client = tokio::spawn(spawn_client(WS_ENDPOINT, CLIENT_ID));

    client.await
}
