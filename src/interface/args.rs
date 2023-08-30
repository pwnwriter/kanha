use crate::interface::splashes::show_splashes;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = show_splashes(), long_about = show_splashes())]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandChoice,
}

#[allow(non_camel_case_types)]
#[derive(Subcommand)]
pub enum CommandChoice {
    /// Returns the HTTP response code of urls
    status(StatusArgs),
}

#[derive(Args)]
pub struct StatusArgs {
    /// A url or a file containing multiple urls
    #[arg(required = true, short, long)]
    pub filename: String,

    /// A list of the ports or seprated by 2 dots
    #[arg(short, long)]
    pub ports: Option<String>,
}
