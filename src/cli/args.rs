use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "fivem-doctor",
    version,
    about = "Static analysis and quality checks for FiveM resources."
)]
pub struct Args {
    /// Path to a FiveM resource
    pub path: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Terminal)]
    pub format: OutputFormat,

    /// Minimum severity to report
    #[arg(long, value_enum, default_value_t = SeverityFilter::Info)]
    pub severity: SeverityFilter,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Terminal,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SeverityFilter {
    Info,
    Warning,
    Error,
}
