mod commands;
mod engine;
mod interface;
mod log;

//  asynchronous entry point where the magic happens :dizzy: 
#[tokio::main]
#[allow(clippy::needless_return)]
async fn main() {
    // Simply await the engine's start method
    engine::start().await;
}
