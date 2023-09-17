use bibe_drink_dispenser::spawn_client;
use tokio::task::JoinError;

const CLIENT_ID: usize = 1;
const SERVER: &str = "ws://127.0.0.1:1690/v1/api/ws";

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    let client = tokio::spawn(spawn_client(CLIENT_ID));

    client.await
}
