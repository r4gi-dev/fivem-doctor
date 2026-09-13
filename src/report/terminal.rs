use crate::diagnostic::{Diagnostic, Severity};

pub fn print(diagnostics: &[Diagnostic]) {
    if diagnostics.is_empty() {
        println!("No problems found.");
        return;
    }

    for diagnostic in diagnostics {
        let severity = match diagnostic.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARNING",
            Severity::Info => "INFO",
        };

        let location = match (diagnostic.line, diagnostic.column) {
            (Some(line), Some(column)) => {
                format!("{}:{}:{}", diagnostic.file, line, column)
            }
            _ => diagnostic.file.clone(),
        };

        println!("{} {} {}", severity, diagnostic.rule_id, location);

        println!("  {}", diagnostic.message);

        if let Some(explanation) = &diagnostic.explanation {
            println!("  Explanation: {}", explanation);
        }

        if let Some(suggestion) = &diagnostic.suggestion {
            println!("  Suggestion: {}", suggestion);
        }

        println!();
    }
}
