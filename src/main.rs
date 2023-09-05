use crate::{
    commands::fuzz::fuzzer::fuzz_url,
    commands::rdns::rev_dns::reverse_dns_lookup,
    commands::status::statuscode::handle_status_command,
    commands::takeover::sub_takeover::subdomain_takeover,
    interface::args::{Cli, CommandChoice},
};
use clap::Parser;

mod commands;
mod interface;
mod log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        CommandChoice::Status(status_args) => handle_status_command(status_args).await?,
        CommandChoice::Fuzzer(fuzz_args) => fuzz_url(fuzz_args).await?,
        CommandChoice::Rdns(rdns_args) => reverse_dns_lookup(rdns_args).await?,
        CommandChoice::Takeover(takeover_args) => subdomain_takeover(takeover_args).await?,
    }

    Ok(())
}

