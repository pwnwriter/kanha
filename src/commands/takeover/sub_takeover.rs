use crate::{commands::kanha_helpers::read_lines, interface::TakeoverArgs};
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

// Guide: https://www.hackerone.com/application-security/guide-subdomain-takeovers
pub async fn subdomain_takeover(
    takeover_args: TakeoverArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(filename) = takeover_args.filename {
        let urls = read_lines(&filename).await?;
        let json_file = takeover_args.json_file;
        let json_file_path = format!("{}", json_file);
        let json_file_contents = tokio::fs::read_to_string(&json_file_path).await?;
        let platform_info: PlatformInfo = serde_json::from_str(&json_file_contents)?;

        let takeover_msg = "Possible subdomain takeover";

        for url_str in urls {
            let url = url_str?.parse::<reqwest::Url>()?;
            let body = reqwest::get(url.clone()).await?.text().await?;

            for platform in &platform_info.platforms {
                let platform_name = &platform.platform;

                match &platform.content {
                    Content::Single(content) => {
                        if body.contains(content) {
                            println!(
                                "{} [{}] -> [{}]",
                                takeover_msg.red().bold(),
                                platform_name.red().bold(),
                                url
                            );
                        }
                    }
                    Content::Multiple(contents) => {
                        for content in contents {
                            if body.contains(content) {
                                println!(
                                    "{} [{}] -> [{}]",
                                    takeover_msg.blue().bold(),
                                    platform_name.red().bold(),
                                    url
                                );
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
