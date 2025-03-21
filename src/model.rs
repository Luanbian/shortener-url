use crate::Result;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool as Pool};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct Shortener {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_url: String,
    pub short_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ShortenerForCreateDb {
    pub user_id: Uuid,
    pub original_url: String,
    pub short_url: String,
}

#[derive(Clone)]
pub struct ModelController {
    pool: Pool,
}

impl ModelController {
    pub async fn new(pool: Pool) -> Result<Self> {
        Ok(Self { pool })
    }
}

impl ModelController {
    pub async fn create_short_link(
        &self,
        short_payload: ShortenerForCreateDb,
    ) -> Result<Shortener> {
        let shortener = Shortener {
            id: Uuid::new_v4(),
            user_id: short_payload.user_id,
            short_url: short_payload.short_url,
            original_url: short_payload.original_url,
        };

        let query =
            "INSERT INTO shorteners (id, user_id, original_url, short_url) VALUES ($1, $2, $3, $4)";

        sqlx::query(query)
            .bind(&shortener.id.to_string())
            .bind(&shortener.user_id.to_string())
            .bind(&shortener.original_url)
            .bind(&shortener.short_url)
            .execute(&self.pool)
            .await?;

        Ok(shortener)
    }

    pub async fn get_original_url(&self, short_url: &str) -> Result<Option<String>> {
        let query = "SELECT original_url FROM shorteners WHERE short_url = $1";
        let result = sqlx::query_scalar::<_, String>(query)
            .bind(short_url)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }
}
