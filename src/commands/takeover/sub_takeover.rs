// Guide: https://www.hackerone.com/application-security/guide-subdomain-takeovers

use crate::{
    commands::kanha_helpers::{read_lines, read_urls_from_stdin},
    interface::TakeoverArgs,
    log::message,
};
use colored::*;
use reqwest;
use serde::{Deserialize, Serialize};

use super::takeover_helper::get_signatures_from_repo;

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
    let platform_info = load_platform_info(&takeover_args.json_file).await?;

    if takeover_args.stdin {
        let urls = read_urls_from_stdin()?;
        process_takeover_urls(&urls, &platform_info).await;
    }
    match (&takeover_args.input.file_path, &takeover_args.input.url) {
        (Some(file_path), None) => {
            let lines = read_lines(file_path).await?;
            let urls: Vec<String> = lines.map_while(Result::ok).collect();
            process_takeover_urls(&urls, &platform_info).await;
        }
        (None, Some(url)) => {
            let urls = vec![url.clone()];
            process_takeover_urls(&urls, &platform_info).await;
        }
        _ => {}
    }
    Ok(())
}

async fn load_platform_info(
    json_file: &Option<String>,
) -> Result<PlatformInfo, Box<dyn std::error::Error>> {
    let json_file_contents = match json_file {
        Some(file) => tokio::fs::read_to_string(file).await?,
        None => {
            message(
                "Seems like you didn't pass the JSON, fetching and using default JSON",
                Color::Blue,
            );
            get_signatures_from_repo().await?
        }
    };
    let platform_info: PlatformInfo = serde_json::from_str(&json_file_contents)?;
    Ok(platform_info)
}

async fn process_takeover_urls(urls: &[String], platform_info: &PlatformInfo) {
    for url_str in urls {
        let url = match reqwest::Url::parse(url_str) {
            Ok(url) => url,
            Err(_) => {
                println!("{} -> [{}]", "Invalid URL".red().bold(), url_str);
                continue;
            }
        };

        let body = reqwest::get(url.clone())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        if let Some(vulnerable_platform) = check_vulnerability(&body, platform_info).await {
            println!(
                "{} [{}] -> [{}]",
                "vulnerable".red().bold(),
                vulnerable_platform.red().bold(),
                url
            );
        } else {
            println!("{} -> [{}]", "Not vulnerable".green().bold(), url);
        }
    }
}

async fn check_vulnerability(body: &str, platform_info: &PlatformInfo) -> Option<String> {
    for platform in &platform_info.platforms {
        let platform_name = &platform.platform;

        match &platform.content {
            Content::Single(content) => {
                if body.contains(content) {
                    return Some(platform_name.clone());
                }
            }
            Content::Multiple(contents) => {
                if contents.iter().any(|content| body.contains(content)) {
                    return Some(platform_name.clone());
                }
            }
        }
    }
    None
}
