use clap::Parser;
use iceberg_compaction::{compact, Config};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    table_path: PathBuf,
    
    #[arg(short, long, default_value_t = 50)]
    compute_hours: u32,
    
    #[arg(short, long)]
    dry_run: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::new(cli.table_path, cli.compute_hours);
    
    let plan = compact(&config, cli.dry_run)?;
    
    println!("Compaction plan generated:");
    println!("Files to compact: {}", plan.files_to_compact.len());
    println!("Estimated runtime: {} minutes", plan.estimated_runtime);
    
    Ok(())
}