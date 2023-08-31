use crate::interface::StatusArgs;
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
                crate::log::success(&format!("{},  [ {} ]", url, response.status()));
            } else {
                crate::log::warn(&format!("{}, [ Failed to fetch ]", url));
            }
        }
    });

    futures::future::join_all(futures).await;
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

        fetch_and_print_status_codes(urls).await;
    } else {
        // Handle file shits
        crate::log::error("No such file or directory");
    }

    Ok(())
}
