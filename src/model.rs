use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool as Pool;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Shortener {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_url: String,
    pub short_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ShortenerForCreate {
    pub user_id: Uuid,
    pub original_url: String,
}

#[derive(Clone)]
pub struct ModelController {
    short_links_list: Arc<Mutex<Vec<Option<Shortener>>>>,
    pool: Pool,
}

impl ModelController {
    pub async fn new(pool: Pool) -> Result<Self> {
        Ok(Self {
            short_links_list: Arc::default(),
            pool,
        })
    }
}

impl ModelController {
    pub async fn create_short_link(&self, short_payload: ShortenerForCreate) -> Result<Shortener> {
        let shortener = Shortener {
            id: Uuid::new_v4(),
            user_id: short_payload.user_id,
            short_url: short_payload.original_url.clone(),
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

    pub async fn list_short_links(&self) -> Result<Vec<Shortener>> {
        let store = self.short_links_list.lock().unwrap();

        let short_links = store.iter().filter_map(|t| t.clone()).collect();

        Ok(short_links)
    }

    pub async fn delete_short_link(&self, id: u64) -> Result<Shortener> {
        let mut store = self.short_links_list.lock().unwrap();

        let short_link = store.get_mut(id as usize).and_then(|t| t.take());

        short_link.ok_or(Error::TicketNotFound { id })
    }
}
