mod commands;
mod interface;
use commands::*;
mod log;

#[tokio::main]
async fn main() {
    crate::commands::statc::Status_code().await;
}
