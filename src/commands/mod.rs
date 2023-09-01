pub mod fuzz;
pub mod rdns;
pub mod status;
pub mod takeover;
pub use fuzz::*;
pub use rdns::*;
pub use status::*;
pub use takeover::*;

#[allow(dead_code)]
#[allow(unused_variables)]
pub mod kanha_helpers {
    use std::fs::File;
    use std::io::{self, BufRead};

    use crate::log::error;

    /// https://doc.rust-lang.org/rust-by-example/std_misc/rile/read_lines.html
    /// Reads a file line by line
    pub async fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
        match File::open(filename) {
            Ok(file) => Ok(io::BufReader::new(file).lines()),
            Err(_) => {
                error("No such file in this location");

                Err(io::Error::new(io::ErrorKind::NotFound, "No such file"))
            }
        }
    }

    /// Adds http(s) protocol to urls if not already
    pub fn add_protocol(urls: Vec<String>) {}

    /// https://www.youtube.com/watch?v=K_wnB9ibCMg&t=1078s
    /// Reads user input from stdin line by line
    pub fn read_urls_from_stdin() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut input = String::new();
        let mut urls = Vec::new();

        loop {
            input.clear();
            match std::io::stdin().lock().read_line(&mut input) {
                Ok(0) => break, // EOF reached
                Ok(_) => urls.push(input.trim().to_string()),
                Err(err) => return Err(Box::new(err)),
            }
        }

        Ok(urls)
    }
}
