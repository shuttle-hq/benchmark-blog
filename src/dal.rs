use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Clone)]
pub struct Dal {
    pool: PgPool,
}

impl Dal {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_blog(&self, _title: &str) -> Result<Blog> {
        // sqlx::query_as("SELECT * FROM blogs WHERE title = $1")
        let blog = sqlx::query_as("SELECT * FROM blogs LIMIT 1")
            // .bind(title)
            .fetch_one(&self.pool)
            .await?;

        Ok(blog)
    }
}

#[derive(FromRow, Serialize)]
pub struct Blog {
    title: String,
    created_at: DateTime<Utc>,
    body: String,
}
