use crate::interface::args::{Cli, CommandChoice};
use clap::Parser;
use std::result::Result;

mod commands;
mod interface;
mod log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        CommandChoice::status(_) => {
            commands::statc::status_code().await;
        }
    }

    Ok(())
}
