pub mod compaction;
pub mod cache;
pub mod models;
pub mod config;

use crate::{compaction::Strategy, models::CompactionPlan};
use anyhow::Result;

pub struct Config {
    pub table_path: std::path::PathBuf,
    pub compute_hours: u32,
    pub max_file_size: u64,
    pub min_input_files: usize,
}

impl Config {
    pub fn new(table_path: std::path::PathBuf, compute_hours: u32) -> Self {
        Self {
            table_path,
            compute_hours,
            max_file_size: 1024 * 1024 * 1024, // 1GB
            min_input_files: 5,
        }
    }
}

pub fn compact(config: &Config, dry_run: bool) -> Result<CompactionPlan> {
    let strategy = Strategy::new(config);
    let plan = strategy.generate_plan()?;
    
    if !dry_run {
        strategy.execute_plan(&plan)?;
    }
    
    Ok(plan)
}