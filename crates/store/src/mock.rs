use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use mockd::{company::company, job, name};

use super::*;

#[derive(Clone)]
pub struct MockStore {
    inner: Arc<Mutex<MockState>>,
}

struct MockState {
    next_id: i64,
    items: HashMap<i64, Item>,
    pending: VecDeque<i64>,
}

impl MockStore {
    pub fn new_with_seed(n: usize) -> Self {
        let mut state = MockState {
            next_id: 1,
            items: HashMap::new(),
            pending: VecDeque::new(),
        };

        for _ in 0..n {
            let id = state.next_id;
            let title = company();
            let body = job::title();
            let item = Item { id, title, body };
            state.items.insert(id, item);
            state.pending.push_back(id);
            state.next_id += 1;
        }

        Self {
            inner: Arc::new(Mutex::new(state)),
        }
    }

    pub fn new() -> Self {
        Self::new_with_seed(10)
    }
}

#[async_trait]
impl Store for MockStore {
    async fn save_item(&self, id: i64, data: &str) -> Result<()> {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let mut s = self.inner.lock().unwrap();
        if let Some(it) = s.items.get_mut(&id) {
            it.body = data.to_string();
        } else {
            let it = Item {
                id,
                title: name::full(),
                body: data.to_string(),
            };
            s.items.insert(id, it);
        }

        s.pending.retain(|&x| x != id);
        Ok(())
    }

    async fn load_pending_changes(&self) -> Result<Vec<i64>> {
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        let s = self.inner.lock().unwrap();
        Ok(s.pending.iter().cloned().collect())
    }

    async fn get_item(&self, id: i64) -> Result<Option<Item>> {
        let s = self.inner.lock().unwrap();
        Ok(s.items.get(&id).cloned())
    }
}
