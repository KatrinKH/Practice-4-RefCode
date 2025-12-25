use reqwest::Client;
use std::time::Duration;

pub struct IssClient {
    client: Client,
    base_url: String,
}

impl IssClient {
    pub fn new(base_url: String, timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent("cassiopeia/1.0")
            .build()
            .unwrap();

        Self { client, base_url }
    }

    pub async fn fetch(&self) -> Result<String, reqwest::Error> {
        self.client.get(&self.base_url).send().await?.text().await
    }
}
