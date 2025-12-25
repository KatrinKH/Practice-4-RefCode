use crate::{clients::iss_client::IssClient, repo::iss_repo::IssRepo};
use crate::domain::iss::IssPosition;
use chrono::Utc;

pub struct IssService {
    client: IssClient,
    repo: IssRepo,
}

impl IssService {
    pub fn new(client: IssClient, repo: IssRepo) -> Self {
        Self { client, repo }
    }

    pub async fn fetch_and_store(&self) -> Result<IssPosition, anyhow::Error> {
        let _raw = self.client.fetch().await?;
        let lat = 0.0;
        let lon = 0.0;

        let fetched_at = Utc::now();
        self.repo.upsert_position(lat, lon, fetched_at).await?;

        Ok(IssPosition { latitude: lat, longitude: lon, fetched_at })
    }
}
