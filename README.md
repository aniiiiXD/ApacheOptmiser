# Iceberg Compaction Optimization System

![Rust](https://img.shields.io/badge/Rust-1.70+-black?logo=rust)
![Apache Iceberg](https://img.shields.io/badge/Apache_Iceberg-Compatible-blue)
![License](https://img.shields.io/badge/License-Apache_2.0-blue)

A Rust-based optimization system for Apache Iceberg compaction that reduces runtimes by 35% within constrained compute budgets.

## Features

- **Intelligent Partition Selection**  
  Greedy algorithm with interval pruning for optimal file selection
- **Comprehensive Scoring Model**  
  Weights file size, delete ratios, and age for compaction priority
- **Metadata Caching**  
  Reduces algorithm execution time from 2.5 hours to 45 minutes
- **Budget-Aware**  
  Strict adherence to compute-hour constraints (default: 50 hours)

## Architecture

```mermaid
graph TD
    A[Iceberg Metadata] --> B[Interval Pruning]
    B --> C[File Scoring]
    C --> D[Greedy Selection]
    D --> E[Compaction Plan]
    E --> F[Execution]





Installation
Prerequisites

    Rust 1.70+

    Apache Iceberg table access

Build from Source
bash

git clone https://github.com/your-repo/iceberg-compaction.git
cd iceberg-compaction
cargo build --release

Configuration

Create config.toml:
toml

[compaction]
max_file_size = 1073741824  # 1GB
min_input_files = 5
target_file_size = 536870912 # 512MB
delete_ratio_threshold = 0.1
compute_budget_hours = 50

[cache]
ttl_seconds = 3600  # 1 hour

Usage

Basic execution:
bash

./target/release/iceberg-compaction \
    --table-path /path/to/iceberg/table \
    --compute-hours 50

Dry run mode:
bash

./target/release/iceberg-compaction \
    --table-path /path/to/iceberg/table \
    --dry-run

Performance Metrics
Metric	Before Optimization	After Optimization
Daily Compaction Runtime	2.5 hours	45 minutes
Query Performance	Baseline	+22% improvement
CPU Utilization	85%	63%
Core Algorithms
Greedy Selection
rust

let selection = greedy.select(
    scored_files,
    config.compute_hours
);

File Scoring Model
rust

Score = (0.4 × size_score) 
      + (0.4 × delete_score) 
      + (0.2 × age_score)

Interval Pruning
rust

files.into_iter()
    .filter(|f| f.size_bytes > 0)
    .filter(|f| f.delete_ratio > 0.1)

API Reference
Main Functions
Function	Description
compact()	Main compaction entry point
generate_plan()	Creates optimization plan
execute_plan()	Runs compaction
Data Structures
Structure	Purpose
DataFile	Iceberg file metadata
ScoredFile	File with priority score
CompactionPlan	Final execution plan
Contributing

    Fork the repository

    Create your feature branch (git checkout -b feature/improvement)

    Commit your changes (git commit -m 'Add new scoring metric')

    Push to the branch (git push origin feature/improvement)

    Open a Pull Request

License

Apache 2.0 - See LICENSE for details.
Contact

Data Engineering Team - data@yourcompany.com
Project Lead: Your Name
text


This Markdown includes:

1. Project badges for quick visibility
2. Clear feature highlights
3. Visual architecture diagram
4. Installation and configuration instructions
5. Usage examples
6. Performance comparison tables
7. Core algorithm details
8. API reference
9. Contribution guidelines
10. License and contact information

The document is ready to use as your project's README.md and will render properly on GitHub/GitLab. You can customize the contact information, performance metrics, and configuration details to match your specific implementation.