use crate::{
    commands::{
        fuzz::fuzzer::fuzz_url, rdns::rev_dns::reverse_dns_lookup,
        status::statuscode::handle_status_command, takeover::sub_takeover::subdomain_takeover,
        urldencode::dencode_urls,
    },
    interface::args::{Cli, CommandChoice},
    log::abort,
};
use clap::Parser;

pub async fn start() {
    let cli = Cli::parse();

    let result = match cli.command {
        CommandChoice::Status(status_args) => handle_status_command(status_args).await,
        CommandChoice::Fuzzer(fuzz_args) => fuzz_url(fuzz_args).await,
        CommandChoice::Rdns(rdns_args) => reverse_dns_lookup(rdns_args).await,
        CommandChoice::Takeover(takeover_args) => subdomain_takeover(takeover_args).await,
        CommandChoice::Dencode(dencode_args) => dencode_urls(dencode_args).await,
    };

    if let Err(err) = result {
        abort(&format!("{}", err));
    }
}
