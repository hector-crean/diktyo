// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     Ok(())
// }

use bibe_desktop_client::{errors, update_bike};

#[tokio::main]
async fn main() -> errors::Result<()> {
    update_bike().await?;
    Ok(())
}
