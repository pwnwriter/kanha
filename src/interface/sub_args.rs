use clap::Args;

#[derive(Debug, Args, Clone)]
#[group(required = false, multiple = false)]
pub struct Input {
    /// A single url
    #[arg(short, long)]
    pub url: Option<String>,

    /// Path of the file containing multiple urls
    #[arg(short, long)]
    pub file_path: Option<String>,
}

#[derive(Args, Clone)]
pub struct StatusArgs {
    /// A file containing multiple urls
    #[arg(short, long)]
    #[arg(required = false, conflicts_with = "stdin")]
    pub filename: Option<String>,

    /// Define the maximum concurrent tasks
    #[arg(short, long, default_value = "20")]
    pub tasks: usize,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,

    /// Define your status code for selective exclusion.
    #[arg(long)]
    pub exclude: Option<String>,
}

#[derive(Args)]
pub struct FuzzerArgs {
    /// A file containing a list of payloads
    #[arg(required = true, short, long)]
    pub payloads: String,

    #[command(flatten)]
    pub input: Input,

    /// Define the maximum concurrent tasks.
    #[arg(short, long, default_value = "20")]
    pub tasks: usize,

    /// Define your status code for selective exclusion.
    #[arg(long)]
    pub exclude: Option<String>,
}

#[derive(Args, Clone)]
pub struct TakeoverArgs {
    #[command(flatten)]
    pub input: Input,

    /// A json file containing signature values of different services
    #[arg(required = false, short, long)]
    pub json_file: String,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,
}

#[derive(Args)]
pub struct RdnsArgs {
    /// a file containing a list of possible wordlists
    #[arg(required = true, conflicts_with = "stdin", short, long)]
    pub filename: Option<String>,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,
}

#[derive(Args)]
pub struct DencodeArgs {
    /// Provide a url to encode
    #[arg(required = false, conflicts_with = "decode", long)]
    pub encode: Option<String>,

    /// Provide a url to dencode
    #[arg(required = false, long)]
    pub decode: Option<String>,
}
