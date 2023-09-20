// Guide :- https://youtu.be/4Jk_I-cw4WE?si=m9hZMwm_5D7FErWY

use crate::{commands::kanha_helpers::read_lines, interface::RedirectArgs};
use colored::Colorize;
use reqwest::Client;
use std::error::Error;

lazy_static::lazy_static! {
    static ref HTTP_CLIENT: Client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to create reqwest client");
}

pub async fn check_openredirect(redirect_args: RedirectArgs) -> Result<(), Box<dyn Error>> {
    let url = redirect_args.url;
    println!("{}", url);

    if let Some(filename) = redirect_args.filename {
        let payloads = read_lines(&filename).await?;

        for payload in payloads {
            match payload {
                Ok(payload_str) => {
                    let target_url = format!("{}{}", &url, payload_str);
                    println!("{}", target_url);

                    let response = HTTP_CLIENT.head(&target_url).send().await?;

                    // Check if the response status code is one of the specified redirection codes
                    if response.status() == reqwest::StatusCode::MOVED_PERMANENTLY
                        || response.status() == reqwest::StatusCode::FOUND
                        || response.status() == reqwest::StatusCode::TEMPORARY_REDIRECT
                        || response.status() == reqwest::StatusCode::PERMANENT_REDIRECT
                    {
                        println!("Vulnerable URL: {}", target_url.red());
                    }
                }
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                    continue; // Skip this line and continue with the next one
                }
            }
        }
    }

    Ok(())
}
