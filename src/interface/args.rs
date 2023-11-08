use crate::interface::{splashes::show_splashes, sub_args::*};
use clap::{Parser, Subcommand};

/// The KANHA CLI.
#[derive(Parser)]
#[command(author, version, about = show_splashes(), long_about = show_splashes())]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// The command to execute.
    #[clap(subcommand)]
    pub command: CommandChoice,
}

#[derive(Subcommand)]
pub enum CommandChoice {
    /// Just return the HTTP response code of urls
    #[command(arg_required_else_help = true)]
    #[clap(name = "status")]
    Status(StatusArgs),

    /// Fuzz urls and return the response codes
    #[command(arg_required_else_help = true)]
    #[clap(name = "fuzz")]
    Fuzzer(FuzzerArgs),

    /// Reverse dns lookup
    #[command(arg_required_else_help = true)]
    #[clap(name = "rdns")]
    Rdns(RdnsArgs),

    /// Check possible subdomain takeover vulnerability
    #[command(arg_required_else_help = true)]
    #[clap(name = "takeover")]
    Takeover(TakeoverArgs),

    /// (De|En) code urls
    #[command(arg_required_else_help = true)]
    #[clap(name = "urldencode")]
    Dencode(DencodeArgs),
}
