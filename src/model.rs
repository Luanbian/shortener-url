use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Shortener {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_url: String,
    pub short_url: String,
}

#[derive(Deserialize)]
pub struct ShortenerForCreate {
    pub user_id: Uuid,
    pub original_url: String,
}

#[derive(Clone)]
pub struct ModelController {
    short_links_list: Arc<Mutex<Vec<Option<Shortener>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            short_links_list: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_short_link(&self, short_payload: ShortenerForCreate) -> Result<Shortener> {
        let mut store = self.short_links_list.lock().unwrap();

        let shortener = Shortener {
            id: Uuid::new_v4(),
            user_id: short_payload.user_id,
            short_url: short_payload.original_url.clone(),
            original_url: short_payload.original_url,
        };

        store.push(Some(shortener.clone()));

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
