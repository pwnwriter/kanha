use crate::interface::TakeoverArgs;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Debug, Deserialize, Serialize)]
struct PlatformInfo {
    platforms: Vec<Platform>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Platform {
    platform: String,
    content: Value,
}

// Guide :- https://www.hackerone.com/application-security/guide-subdomain-takeovers
pub async fn subdomain_takeover(
    takeover_args: TakeoverArgs,
) -> Result<(), Box<dyn std::error::Error>> {
        let json_file = takeover_args.json_file;
        let json_file_path = format!("{}", json_file);
        println!("{}", json_file_path);
        let json_file_contents = tokio::fs::read_to_string(&json_file_path).await?;
        let platform_info: PlatformInfo = serde_json::from_str(&json_file_contents)?;

        println!("{:?}", platform_info);

        let body = reqwest::get("https://letscheckthissite.github.io")
            .await?
            .text()
            .await?;

        if body.contains("<p><strong>There isn't a GitHub Pages site here.</strong></p>") {
            println!("Possible subdomain takeover: ");
        

        // Check if the response body contains the content of any platform
    }
    Ok(())
}
