use crate::{
    commands::{
        kanha_helpers::read_lines,
        statuscode::{fetch_and_print_status_codes, ArgsWithTasks},
    },
    interface::FuzzerArgs,
    log::abort,
};
use futures::stream::iter;
use futures::StreamExt;

impl ArgsWithTasks for FuzzerArgs {
    fn tasks(&self) -> usize {
        self.tasks
    }
}

pub async fn fuzz_url(fuzzer_args: FuzzerArgs) -> Result<(), Box<dyn std::error::Error>> {
    let url_to_fuzz = &fuzzer_args.url;

    let lines = read_lines(&fuzzer_args.wordlist).await?;

    if !url_to_fuzz.contains("FUZZ") {
        abort("The URL must contain the 'FUZZ' keyword for fuzzing.")
    }

    // https://users.rust-lang.org/t/how-far-to-take-iterators-to-avoid-for-loops/30167
    // Takes a collection of lines, filters out valid results, and replaces a placeholder ("FUZZ") with each valid line to create a list of formatted_urls
    let formatted_urls: Vec<String> = lines
        .map_while(Result::ok)
        .map(|line| url_to_fuzz.replace("FUZZ", &line))
        .collect();

    if let Some(exclude_str) = fuzzer_args.exclude.clone() {
        let exclude_codes: Vec<usize> = exclude_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        fetch_and_print_status_codes_with_exclude(formatted_urls, fuzzer_args, exclude_codes).await;
    } else {
        fetch_and_print_status_codes(formatted_urls, fuzzer_args).await;
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
