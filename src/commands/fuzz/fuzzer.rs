use crate::{
    commands::{
        kanha_helpers::read_lines,
        statuscode::{fetch_and_print_status_codes, ArgsWithTasks},
    },
    interface::FuzzerArgs,
    log::abort,
};
use anyhow::{Context, Result};
use futures::stream::iter;
use futures::StreamExt;

impl ArgsWithTasks for FuzzerArgs {
    fn tasks(&self) -> usize {
        self.tasks
    }
}

pub async fn fuzz_url(fuzzer_args: FuzzerArgs) -> Result<(), Box<dyn std::error::Error>> {
    let urls_to_fuzz: Vec<String> = match (&fuzzer_args.input.url, &fuzzer_args.input.file_path) {
        (Some(url), None) => vec![url.clone()],
        (None, Some(file_path)) => read_lines(file_path)
            .await
            .context(format!("Error reading URLs from file: {:?}", file_path))?
            .filter_map(Result::ok)
            .collect(),
        _ => unreachable!(),
    };

    if urls_to_fuzz.is_empty() {
        abort("No URLs to fuzz.");
    }

    let payloads = read_lines(&fuzzer_args.payloads)
        .await
        .context("Error reading payloads from wordlist file")?;

    let formatted_urls: Vec<String> = payloads
        .map_while(Result::ok)
        .flat_map(|line| {
            urls_to_fuzz
                .iter()
                .map(move |url| url.replace("FUZZ", &line))
        })
        .collect();

    if let Some(exclude_str) = fuzzer_args.exclude.clone() {
        let exclude_codes: Vec<usize> = exclude_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        fetch_and_print_status_codes_with_exclude(
            formatted_urls.clone(),
            fuzzer_args,
            exclude_codes,
        )
        .await
    } else {
        fetch_and_print_status_codes(formatted_urls.clone(), fuzzer_args).await
    }

    Ok(())
}

pub async fn fetch_and_print_status_codes_with_exclude(
    urls: Vec<String>,
    fuzzer_args: FuzzerArgs,
    exclude: Vec<usize>,
) {
    let tasks = fuzzer_args.tasks();

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
