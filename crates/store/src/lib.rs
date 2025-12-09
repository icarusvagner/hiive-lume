mod mock;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub title: String,
    pub body: String,
}

#[async_trait]
pub trait Store: Send + Sync + 'static {
    async fn save_item(&self, id: i64, data: &str) -> Result<()>;
    async fn load_pending_changes(&self) -> Result<Vec<i64>>;
    async fn get_item(&self, id: i64) -> Result<Option<Item>>;
}

pub type StoreRef = Arc<dyn Store>;

pub use mock::*;
