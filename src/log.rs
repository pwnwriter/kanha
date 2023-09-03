use colored::{Color, Colorize};

/// Prints the given message to the console and aborts the process.
#[allow(dead_code)]
pub fn abort(msg: &str) -> ! {
    error(msg);
    std::process::exit(1);
}

#[allow(dead_code)]
pub fn info(msg: &str, color: Color) {
    println!("{}: {}", "Info".bold().color(color), msg);
}

pub fn error(msg: &str) {
    println!("{}: {}", "Error".bold().color(Color::Red), msg);
}

#[allow(dead_code)]
pub fn success(msg: &str) {
    println!("{}: {}", "Success".bold().color(Color::Green), msg);
}

#[allow(dead_code)]
pub fn warn(msg: &str) {
    println!("{}: {}", "Warning".bold().color(Color::Yellow), msg);
}
