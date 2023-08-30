mod commands;
mod interface;
mod log;

#[tokio::main]
async fn main() {
    crate::commands::statc::status_code().await;
}
