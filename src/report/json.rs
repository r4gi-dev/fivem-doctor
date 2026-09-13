use crate::diagnostic::Diagnostic;

pub fn print(diagnostics: &[Diagnostic]) -> anyhow::Result<()> {
    let output = serde_json::to_string_pretty(diagnostics)?;
    println!("{output}");
    Ok(())
}
