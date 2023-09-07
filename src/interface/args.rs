use crate::interface::splashes::show_splashes;
use clap::{Args, Parser, Subcommand};

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
#[command(arg_required_else_help = true)]
#[command(author, version, about = show_splashes(), long_about = show_splashes())]
pub enum CommandChoice {
    /// Just return the HTTP response code of URLs
    #[command(arg_required_else_help = true)]
    #[clap(name = "status")]
    Status(StatusArgs),

    /// Fuzz a URL and return the response codes
    #[command(arg_required_else_help = true)]
    #[clap(name = "fuzz")]
    Fuzzer(FuzzerArgs),

    /// Reverse dns lookup
    #[command(arg_required_else_help = true)]
    #[clap(name = "rdns")]
    Rdns(RdnsArgs),

    /// Check possible subdomain takeover
    #[command(arg_required_else_help = true)]
    #[clap(name = "takeover")]
    Takeover(TakeoverArgs),
}

#[derive(Args)]
pub struct StatusArgs {
    /// A url or a file containing multiple urls
    #[arg(required = false, short, long)]
    pub filename: Option<String>,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,

    /// Define the maximum concurrent tasks
    #[arg(short, long, default_value = "10")]
    pub tasks: usize,
}

#[derive(Args)]
pub struct FuzzerArgs {
    /// A file containing a list of possible wordlists
    #[arg(required = true, short, long)]
    pub wordlist: String,

    /// Provide a url to fuzz
    #[arg(required = true, short, long)]
    pub url: String,

    /// Define the maximum concurrent tasks
    #[arg(short, long, default_value = "10")]
    pub tasks: usize,
}

#[derive(Args)]
pub struct RdnsArgs {
    /// a file containing a list of possible wordlists
    #[arg(required = true, short, long)]
    pub filename: String,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,
}

#[derive(Args, Clone)]
pub struct TakeoverArgs {
    /// A json file containing signature values of different services
    #[arg(required = true, short, long)]
    pub json_file: String,

    /// A file containing a list of urls
    #[arg(required = false, short, long)]
    pub filename: Option<String>,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,
}
