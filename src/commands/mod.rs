pub mod fuzz;
pub mod status;
pub use fuzz::*;
pub use status::*;

#[allow(dead_code)]
#[allow(unused_variables)]
pub mod kanha_helpers {
    use std::fs::File;
    use std::io::{self, BufRead};

    use crate::log::error;

    // https://doc.rust-lang.org/rust-by-example/std_misc/rile/read_lines.html
    pub async fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
        match File::open(filename) {
            Ok(file) => Ok(io::BufReader::new(file).lines()),
            Err(_) => {
                error("No such file in this location");

                Err(io::Error::new(io::ErrorKind::NotFound, "No such file"))
            }
        }
    }

    pub fn add_protocol(urls: Vec<String>) {

        
    }
}
