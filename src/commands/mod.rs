pub mod fuzz;
pub mod rdns;
pub mod status;
pub mod takeover;
pub mod urldencode;
pub use {fuzz::*, rdns::*, status::*};

#[allow(dead_code)]
pub mod kanha_helpers {
    use crate::log::abort;
    use std::fs::File;
    use std::io::{self, BufRead};

    /// https://doc.rust-lang.org/rust-by-example/std_misc/rile/read_lines.html
    /// Reads a file line by line and returns an iterator for reading those lines, or aborts with a message if the file doesn't exist.
    #[inline]
    pub async fn read_lines(filename: &str) -> anyhow::Result<io::Lines<io::BufReader<File>>> {
        match File::open(filename) {
            Ok(file) => Ok(io::BufReader::new(file).lines()),
            Err(_) => {
                abort("No such file in this location");
            }
        }
    }

    /// https://www.youtube.com/watch?v=K_wnB9ibCMg&t=1078s
    /// Reads lines from the standard input, collects them into a Vec<String> using the collect method, and returns the result
    #[inline]
    pub fn read_urls_from_stdin() -> anyhow::Result<Vec<String>> {
        Ok(io::read_to_string(io::stdin().lock())?
            .lines()
            .map(|url| url.trim().to_owned())
            .collect())
    }

    #[inline]
    pub fn print_info() {}
}
