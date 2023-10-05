// Guide: https://www.hackerone.com/application-security/guide-subdomain-takeovers

use crate::{
    commands::kanha_helpers::{read_lines, read_urls_from_stdin},
    interface::TakeoverArgs,
};
use colored::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct PlatformInfo {
    platforms: Vec<Platform>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Platform {
    platform: String,
    content: Content,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Content {
    Single(String),
    Multiple(Vec<String>),
}

pub async fn subdomain_takeover(
    takeover_args: TakeoverArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    match takeover_args.stdin {
        true => {
            let urls = read_urls_from_stdin()?;
            process_takeover_urls(urls, &takeover_args.clone()).await; // Clone the args
        }
        false => {
            if let Some(filename) = &takeover_args.filename {
                let lines = read_lines(filename).await?;
                let urls: Vec<String> = lines
                    .map_while(Result::ok) // Filter out lines with read errors
                    .collect();

                process_takeover_urls(urls, &takeover_args).await;
            }
        }
    }

    Ok(())
}

async fn check_vulnerability(
    body: &str,
    contents: &[String],
    platform_name: &str,
    url: &reqwest::Url,
) -> bool {
    for content in contents {
        if body.contains(content) {
            println!(
                "{} [{}] -> [{}]",
                "vulnerable".blue().bold(),
                platform_name.red().bold(),
                url
            );
            return true;
        }
    }
    false
}

async fn process_takeover_urls(urls: Vec<String>, takeover_args: &TakeoverArgs) {
    let json_file_path = takeover_args.json_file.to_string();
    let json_file_contents = tokio::fs::read_to_string(&json_file_path).await.unwrap();
    let platform_info: PlatformInfo = serde_json::from_str(&json_file_contents).unwrap();

    for url_str in urls {
        let url = url_str.parse::<reqwest::Url>().unwrap();
        let body = reqwest::get(url.clone())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut vulnerable = false;

        for platform in &platform_info.platforms {
            let platform_name = &platform.platform;

            match &platform.content {
                Content::Single(content) => {
                    if body.contains(content) {
                        println!(
                            "{} [{}] -> [{}]",
                            "vulnerable".red().bold(),
                            platform_name.red().bold(),
                            url
                        );
                        vulnerable = true;
                        break;
                    }
                }
                Content::Multiple(contents) => {
                    if check_vulnerability(&body, contents, platform_name, &url).await {
                        vulnerable = true;
                        break;
                    }
                }
            }
        }

        if !vulnerable {
            println!("{} -> [{}]", "Not vulnerable".green().bold(), url,);
        }
    }
}
