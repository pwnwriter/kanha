mod commands;
mod engine;
mod interface;
mod log;

//  asynchronous entry point where the magic happens :dizzy: 
#[tokio::main]
async fn main() {
    engine::start().await;
}
