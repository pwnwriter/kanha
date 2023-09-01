use crate::interface::StatusArgs;
use reqwest::Client;
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
                println!("{} [{}]", url, response.status().as_u16()); // Print status code as integer
            }
        }
    });

    futures::future::join_all(tasks).await;
}

pub async fn handle_status_command(
    status_args: StatusArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    match status_args.stdin {
        true => {
            let urls = crate::commands::kanha_helpers::read_urls_from_stdin()?;
            fetch_and_print_status_codes(urls, status_args).await;
        }
        false => {
            if let Some(filename) = &status_args.filename {
                if let Ok(lines) = crate::commands::kanha_helpers::read_lines(filename).await {
                    let urls: Vec<String> = lines
                        .map_while(Result::ok) // Filter out lines with read errors
                        .collect();

                    fetch_and_print_status_codes(urls, status_args).await;
                }
            }
        }
    }

    Ok(())
}
