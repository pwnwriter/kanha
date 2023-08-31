use crate::interface::StatusArgs;
use reqwest::Client;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Arc;
use tokio::sync::Semaphore;

// Reuse the reqwest client instance
lazy_static::lazy_static! {
    static ref HTTP_CLIENT: Client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to create reqwest client");
}

pub async fn fetch_and_print_status_codes(urls: Vec<String>, status_args: StatusArgs) {
    let client = &HTTP_CLIENT;
    let semaphore = Arc::new(Semaphore::new(status_args.tasks));

    let tasks = urls.into_iter().map(|url| {
        let client = client.clone();
        let semaphore = semaphore.clone();
        async move {
            let _permit = semaphore.acquire().await.expect("Semaphore error");
            if let Ok(response) = client.get(&url).send().await {
                crate::log::success(&format!("{},  [ {} ]", url, response.status()));
            } else {
                crate::log::warn(&format!("{}, [ Failed to fetch ]", url));
            }
        }
    });

    futures::future::join_all(tasks).await;
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub async fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub async fn handle_status_command(
    status_args: StatusArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(lines) = read_lines(&status_args.filename).await {
        let urls: Vec<String> = lines
            .map_while(Result::ok) // Filter out lines with read errors
            .collect();

        fetch_and_print_status_codes(urls, status_args).await; 
    } else {
        // Handle file error shits
        crate::log::error("No such file or directory");
    }

    Ok(())
}
