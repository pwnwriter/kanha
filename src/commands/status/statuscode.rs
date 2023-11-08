use crate::{
    commands::kanha_helpers::{read_lines, read_urls_from_stdin},
    interface::StatusArgs,
};
use futures::stream::iter;
use futures::StreamExt;
use {reqwest::Client, std::sync::Arc, tokio::sync::Semaphore};

// Reuse the reqwest client instance
lazy_static::lazy_static! {
    static ref HTTP_CLIENT: Client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to create reqwest client");
}

pub async fn fetch_and_print_status_codes<T>(urls: Vec<String>, args: T)
where
    T: ArgsWithTasks,
{
    let client = &HTTP_CLIENT;
    let semaphore = Arc::new(Semaphore::new(args.tasks()));

    let tasks = urls.into_iter().map(|url| {
        let semaphore = semaphore.clone();
        async move {
            let _permit = semaphore.acquire().await.expect("Semaphore error");
            if let Ok(response) = client.get(&url).send().await {
                println!("{} [{}]", url, response.status().as_u16()); // Print status code as an integer
            }
        }
    });

    futures::future::join_all(tasks).await;
}

pub trait ArgsWithTasks {
    fn tasks(&self) -> usize;
}

impl ArgsWithTasks for StatusArgs {
    fn tasks(&self) -> usize {
        self.tasks
    }
}

pub async fn handle_status_command(
    status_args: StatusArgs,
) -> anyhow::Result<(), Box<dyn std::error::Error>> {
    match status_args.stdin {
        true => {
            let urls = read_urls_from_stdin()?;

            if let Some(exclude_str) = status_args.exclude.clone() {
                let exclude_codes: Vec<usize> = exclude_str
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                fetch_and_print_status_codes_with_exclude(urls, status_args, exclude_codes).await;
            } else {
                fetch_and_print_status_codes(urls, status_args).await;
            }
        }
        false => {
            if let Some(filename) = &status_args.filename {
                if let Ok(lines) = read_lines(filename).await {
                    let urls: Vec<String> = lines
                        .map_while(Result::ok) // Filter out lines with read errors
                        .collect();

                    if let Some(exclude_str) = status_args.exclude.clone() {
                        let exclude_codes: Vec<usize> = exclude_str
                            .split(',')
                            .filter_map(|s| s.trim().parse().ok())
                            .collect();

                        fetch_and_print_status_codes_with_exclude(urls, status_args, exclude_codes)
                            .await;
                    } else {
                        fetch_and_print_status_codes(urls, status_args).await;
                    }
                }
            }
        }
    }

    Ok(())
}

pub async fn fetch_and_print_status_codes_with_exclude(
    urls: Vec<String>,
    status_args: StatusArgs,
    exclude: Vec<usize>,
) {
    let tasks = status_args.tasks();

    let client = reqwest::Client::new();

    let responses = iter(urls.into_iter())
        .map(|url| {
            let client = &client;
            async move {
                let response = client.get(&url).send().await;
                (url, response)
            }
        })
        .buffer_unordered(tasks)
        .collect::<Vec<(String, Result<reqwest::Response, reqwest::Error>)>>()
        .await;

    for (url, response_result) in responses {
        match response_result {
            Ok(response) => {
                let status_code = response.status();
                let status_code_usize: usize = match status_code.as_u16().try_into() {
                    Ok(code) => code,
                    Err(_) => {
                        eprintln!("Error fetching URL {}: Invalid status code", url);
                        continue;
                    }
                };
                if !exclude.contains(&status_code_usize) {
                    println!("URL: {}, Status Code: {}", url, status_code_usize);
                }
            }
            Err(err) => {
                eprintln!("Error fetching URL {}: {}", url, err);
            }
        }
    }
}
