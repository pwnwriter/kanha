pub mod sub_takeover;

pub mod takeover_helper {

    use crate::log::abort;

    const JSON_URL: &str = "https://raw.githubusercontent.com/Pentester-Nepal/Subdomain-Takeover-Signatures/main/signatures.json";

    pub async fn get_signatures_from_repo() -> Result<String, reqwest::Error> {
        let response = reqwest::get(JSON_URL).await?;

        if response.status().is_success() {
            let json_text = response.text().await?;
            Ok(json_text)
        } else {
            abort("Couldn't fetch default json, parse manualy");
        }
    }
}
