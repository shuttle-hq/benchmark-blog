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

    /// Get a single blog with the given title
    pub async fn get_blog(&self, title: &str) -> Result<Blog> {
        let blog = sqlx::query_as("SELECT * FROM blogs WHERE title = $1")
            .bind(title)
            .fetch_one(&self.pool)
            .await?;

        Ok(blog)
    }

    /// Get all blogs
    pub async fn get_blogs(&self) -> Result<Vec<Blog>> {
        let blog = sqlx::query_as("SELECT * FROM blogs")
            .fetch_all(&self.pool)
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
