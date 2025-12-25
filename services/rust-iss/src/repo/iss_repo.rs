use sqlx::PgPool;
use chrono::{DateTime, Utc};

pub struct IssRepo {
    pool: PgPool,
}

impl IssRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert_position(
        &self,
        lat: f64,
        lon: f64,
        fetched_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO iss_positions (latitude, longitude, fetched_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (fetched_at)
            DO UPDATE SET
                latitude = EXCLUDED.latitude,
                longitude = EXCLUDED.longitude
            "#,
            lat,
            lon,
            fetched_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
