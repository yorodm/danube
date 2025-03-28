use danube_core::storage::{Segment, StorageBackend};
use moka::future::Cache as MokaCache;
use std::sync::Arc;
use tokio::{sync::RwLock, time::Duration};

use crate::errors::{ReliableDispatchError, Result};

#[derive(Debug, Clone)]
pub struct TopicCache {
    // Primary fast memory cache
    memory_cache: MokaCache<String, Arc<RwLock<Segment>>>,
    // Storage backend for segments
    storage: Arc<dyn StorageBackend>,
}

impl TopicCache {
    pub fn new(storage: Arc<dyn StorageBackend>, max_capacity: u64, idle_time: u64) -> Self {
        let memory_cache = MokaCache::builder()
            // Max 100 segment entries
            .max_capacity(max_capacity)
            // Time to idle (TTI):  10 minutes
            // A cached entry will be expired after the specified duration past from get or insert.
            .time_to_idle(Duration::from_secs(idle_time * 60))
            // Create the cache.
            .build();

        Self {
            memory_cache,
            storage,
        }
    }

    pub async fn get_segment(
        &self,
        topic_name: &str,
        id: usize,
    ) -> Result<Option<Arc<RwLock<Segment>>>> {
        let key = format!("{}:{}", topic_name, id);

        // Try memory cache first
        if let Some(segment) = self.memory_cache.get(&key).await {
            return Ok(Some(segment));
        }

        // Try storage backend
        match self.storage.get_segment(topic_name, id).await {
            Ok(segment) => {
                // Update memory cache
                if let Some(ref segment) = segment {
                    self.memory_cache.insert(key, segment.clone()).await;
                }
                Ok(segment)
            }
            Err(e) => Err(ReliableDispatchError::StorageError(e.to_string())),
        }
    }

    pub async fn put_segment(
        &self,
        topic_name: &str,
        id: usize,
        segment: Arc<RwLock<Segment>>,
    ) -> Result<()> {
        let key = format!("{}:{}", topic_name, id);

        // Always update memory cache
        self.memory_cache.insert(key, segment.clone()).await;

        // Only store in backend if segment is closed
        let is_closed = {
            let segment_guard = segment.read().await;
            segment_guard.close_time > 0
        };

        if is_closed {
            // Update storage backend only for closed segments
            self.storage.put_segment(topic_name, id, segment).await?;
        }

        Ok(())
    }

    pub async fn remove_segment(&self, topic_name: &str, id: usize) -> Result<()> {
        let key = format!("{}:{}", topic_name, id);

        // Remove from memory cache
        self.memory_cache.remove(&key).await;

        // Remove from storage backend
        self.storage.remove_segment(topic_name, id).await?;
        Ok(())
    }
}
