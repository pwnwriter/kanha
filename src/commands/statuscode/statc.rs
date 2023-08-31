use crate::interface::args::*;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Arc;

#[allow(non_snake_case)]
pub async fn fetch_and_print_status_codes(urls: Vec<String>) {
    let client_builder = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none());
    let client_result = client_builder.build();

    let client = match client_result {
        Ok(client) => Arc::new(client),
        Err(error) => {
            eprintln!("[ERROR] Failed to create reqwest client: {}", error);
            return;
        }
    };

    let futures = urls.into_iter().map(|url| {
        let client = Arc::clone(&client);
        async move {
            if let Ok(response) = client.get(&url).send().await {
                crate::log::info_success(&format!("{},  [ {} ]", url, response.status()));
            } else {
                crate::log::info_error(&format!("{}, [ Failed to fetch ]", url));
            }
        }
    });

    futures::future::join_all(futures).await;
}

pub async fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub async fn status_code() {
    let cli = Cli::parse();
    match cli.command {
        CommandChoice::status(status_args) => {
            if let Ok(lines) = read_lines(&status_args.filename).await {
                let urls: Vec<String> = lines
                    .map_while(Result::ok)// Filter out lines with read errors
                    .collect();

                fetch_and_print_status_codes(urls).await;
            } else {
                // Handle file shits
                crate::log::info_error("No such file or directory");
            }
        }
    }
}
