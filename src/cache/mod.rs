mod partition;

use std::sync::Arc;
use std::collections::HashMap;
use parking_lot::RwLock;
use crate::models::PartitionMetadata;

pub struct MetadataCache {
    partitions: RwLock<HashMap<String, Arc<PartitionMetadata>>>,
    ttl_seconds: u64,
}

impl MetadataCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            partitions: RwLock::new(HashMap::new()),
            ttl_seconds,
        }
    }
    
    pub fn get(&self, partition_id: &str) -> Option<Arc<PartitionMetadata>> {
        self.partitions.read().get(partition_id).cloned()
    }
    
    pub fn put(&self, partition_id: String, metadata: PartitionMetadata) {
        self.partitions.write().insert(partition_id, Arc::new(metadata));
    }
    
    pub fn invalidate(&self, partition_id: &str) {
        self.partitions.write().remove(partition_id);
    }
}