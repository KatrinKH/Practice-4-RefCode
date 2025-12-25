use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct IssPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub fetched_at: DateTime<Utc>,
}
