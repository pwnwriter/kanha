use colored::Colorize;

/// Prints the given message to the console and aborts the process.
pub fn abort(msg: &str) -> ! {
    eprintln!("{}: {msg}", "abort".bold().red());
    std::process::exit(1);
}

pub fn info_error(msg: &str) {
    println!("{}: {msg}", "info".bold().red());
}

pub fn info_success(msg: &str) {
    println!("{}: {msg}", "info".bold().blue());
}
