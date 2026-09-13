mod analyzer;
mod cli;
mod diagnostic;
mod report;
mod rules;

use anyhow::Result;
use clap::Parser;
use cli::args::{Args, OutputFormat, SeverityFilter};
use diagnostic::{Diagnostic, Severity};

fn main() -> Result<()> {
    let args = Args::try_parse().unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2);
    });

    let analyzer = analyzer::resource::ResourceAnalyzer::new(&args.path);

    let diagnostics = match analyzer.analyze() {
        Ok(diagnostics) => diagnostics,
        Err(error) => {
            eprintln!("{error:#}");
            std::process::exit(3);
        }
    };

    let diagnostics = filter_diagnostics(diagnostics, args.severity);

    match args.format {
        OutputFormat::Terminal => report::terminal::print(&diagnostics),
        OutputFormat::Json => report::json::print(&diagnostics)?,
    }

    if !diagnostics.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}

fn filter_diagnostics(diagnostics: Vec<Diagnostic>, minimum: SeverityFilter) -> Vec<Diagnostic> {
    diagnostics
        .into_iter()
        .filter(|diagnostic| match minimum {
            SeverityFilter::Info => true,
            SeverityFilter::Warning => {
                matches!(diagnostic.severity, Severity::Warning | Severity::Error)
            }
            SeverityFilter::Error => diagnostic.severity == Severity::Error,
        })
        .collect()
}
