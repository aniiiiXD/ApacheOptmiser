use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFile {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub record_count: u64,
    pub delete_ratio: f32,
    pub partition_id: String,
    pub age_days: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoredFile {
    pub file: DataFile,
    pub score: f32,
    pub processing_cost: f32,
}

#[derive(Debug)]
pub struct CompactionPlan {
    pub files_to_compact: Vec<ScoredFile>,
    pub estimated_runtime: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionMetadata {
    pub id: String,
    pub files: Vec<DataFile>,
    pub last_updated: u64,
}

// Implement ordering for ScoredFile to work with BinaryHeap
impl PartialOrd for ScoredFile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for ScoredFile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for ScoredFile {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for ScoredFile {}