use std::time::Duration;

use anyhow::anyhow;
use reqwest::{Client, Response, StatusCode, Url};

pub struct RequestHandler {
    client: Client,
}

impl RequestHandler {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent("Mozilla/5.0 Firefox/152.0")
            .build()
            .unwrap_or_default();

        Self { client }
    }
}

impl Default for RequestHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestHandler {
    pub async fn request(&self, url: Url) -> Result<Response, anyhow::Error> {
        print!("Sending request to {} {}", url, "...");
        let response = self.client.get(url.clone()).send().await?;
        let status = response.status();
        println!(
            "\rSending request to {} {} {}",
            url,
            status.as_str(),
            status
                .canonical_reason()
                .ok_or(anyhow!("Error getting response code."))?
        );

        Ok(response)
    }
}
