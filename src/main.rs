use {
    crate::{
        commands::{
            fuzz::fuzzer::fuzz_url, openredirect::check_openredirect,
            rdns::rev_dns::reverse_dns_lookup, status::statuscode::handle_status_command,
            takeover::sub_takeover::subdomain_takeover, urldencode::dencode_urls,
        },
        interface::args::{Cli, CommandChoice},
    },
    clap::Parser,
};

mod commands;
mod interface;
mod log;

//  asynchronous entry point where the magic happens :dizzy: 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let result = match cli.command {
        CommandChoice::Status(status_args) => handle_status_command(status_args).await,
        CommandChoice::Fuzzer(fuzz_args) => fuzz_url(fuzz_args).await,
        CommandChoice::Rdns(rdns_args) => reverse_dns_lookup(rdns_args).await,
        CommandChoice::Takeover(takeover_args) => subdomain_takeover(takeover_args).await,
        CommandChoice::Dencode(dencode_args) => dencode_urls(dencode_args).await,
        CommandChoice::Openredirect(redirect_args) => check_openredirect(redirect_args).await,
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    Ok(())
}
