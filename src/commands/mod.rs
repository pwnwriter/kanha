pub mod fuzz;
pub mod rdns;
pub mod status;
pub mod takeover;
pub use {fuzz::*, rdns::*, status::*, takeover::*};

#[allow(dead_code)]
#[allow(unused_variables)]
pub mod kanha_helpers {
    use crate::log::abort;
    use std::fs::File;
    use std::io::{self, BufRead};

    /// https://doc.rust-lang.org/rust-by-example/std_misc/rile/read_lines.html
    /// Reads a file line by line

    pub async fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
        match File::open(filename) {
            Ok(file) => Ok(io::BufReader::new(file).lines()),
            Err(_) => {
                abort("No such file in this location");
            }
        }
    }

    /// Adds http(s) protocol to urls if not already
    pub fn add_protocol(urls: Vec<String>) {}

    /// https://www.youtube.com/watch?v=K_wnB9ibCMg&t=1078s
    /// Reads user input from stdin line by line
    pub fn read_urls_from_stdin() -> io::Result<Vec<String>> {
        let mut urls = Vec::new();
        let stdin = io::stdin();
        let locked_stdin = stdin.lock();
        for line in locked_stdin.lines() {
            match line {
                Ok(line_content) => urls.push(line_content),
                Err(err) => abort("Failed to read from stdin"),
            }
        }
        Ok(urls)
    }

    pub fn cheatsheet() {}
}
