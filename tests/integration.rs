use std::path::Path;
use std::process::{Command, Output};

use serde_json::Value;

fn run_fixture(fixture: &str) -> Output {
    run_fixture_with_args(fixture, &[])
}

fn run_fixture_with_args(fixture: &str, args: &[&str]) -> Output {
    let path = Path::new("tests").join("fixtures").join(fixture);

    Command::new(env!("CARGO_BIN_EXE_fivem-doctor"))
        .arg(path)
        .args(args)
        .output()
        .expect("failed to execute fivem-doctor")
}

fn run_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_fivem-doctor"))
        .args(args)
        .output()
        .expect("failed to execute fivem-doctor")
}

fn assert_exit_code(output: &Output, expected: i32) {
    assert_eq!(
        output.status.code(),
        Some(expected),
        "expected exit code {expected}, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn parse_json_output(output: &Output) -> Value {
    let stdout = String::from_utf8_lossy(&output.stdout);

    serde_json::from_str(&stdout)
        .unwrap_or_else(|error| panic!("expected valid JSON output: {error}\nstdout:\n{stdout}"))
}

#[test]
fn valid_resource_has_no_diagnostics() {
    let output = run_fixture("valid-resource");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No problems found."));
}

#[test]
fn missing_manifest_reports_f001() {
    let output = run_fixture("missing-manifest");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F001"));
    assert!(stdout.contains("Resource manifest not found"));
}

#[test]
fn missing_file_reports_f002() {
    let output = run_fixture("missing-file");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F002"));
    assert!(stdout.contains("Referenced file does not exist"));
}

#[test]
fn deprecated_lua54_reports_f003() {
    let output = run_fixture("deprecated-lua54");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F003"));
    assert!(stdout.contains("lua54"));
}

#[test]
fn performance_reports_f005() {
    let output = run_fixture("performance");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F005"));
    assert!(stdout.contains("Wait(0)"));
}

#[test]
fn debug_print_reports_f006() {
    let output = run_fixture("debug-print");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F006"));
    assert!(stdout.contains("print()"));
}

#[test]
fn unsafe_event_reports_f004() {
    let output = run_fixture("unsafe-event");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F004"));
    assert!(stdout.contains("unsafe network event"));
}

#[test]
fn safe_event_has_no_f004() {
    let output = run_fixture("safe-event");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("F004"));
}

#[test]
fn privileged_event_reports_f007() {
    let output = run_fixture("privileged-event");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F007"));
    assert!(stdout.contains("authorization"));
}

#[test]
fn authorized_event_has_no_f007() {
    let output = run_fixture("authorized-event");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("F007"));
}

#[test]
fn client_controlled_event_reports_f008() {
    let output = run_fixture("client-controlled-event");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F008"));
    assert!(stdout.contains("Client-controlled value"));
}

#[test]
fn server_event_without_arguments_has_no_f008() {
    let output = run_fixture("server-event-no-args");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("F008"));
}

#[test]
fn info_diagnostic_fails_with_info_severity() {
    let output = run_fixture_with_args("debug-print", &["--severity", "info"]);

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F006"));
}

#[test]
fn info_diagnostic_is_filtered_with_warning_severity() {
    let output = run_fixture_with_args("debug-print", &["--severity", "warning"]);

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No problems found."));
    assert!(!stdout.contains("F006"));
}

#[test]
fn warning_diagnostic_fails_with_warning_severity() {
    let output = run_fixture_with_args("performance", &["--severity", "warning"]);

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F005"));
}

#[test]
fn warning_diagnostic_is_filtered_with_error_severity() {
    let output = run_fixture_with_args("performance", &["--severity", "error"]);

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No problems found."));
    assert!(!stdout.contains("F005"));
}

#[test]
fn error_diagnostic_fails_with_error_severity() {
    let output = run_fixture_with_args("missing-file", &["--severity", "error"]);

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F002"));
}

#[test]
fn error_diagnostic_fails_with_warning_severity() {
    let output = run_fixture_with_args("missing-file", &["--severity", "warning"]);

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("F002"));
}

#[test]
fn json_output_is_valid_and_contains_diagnostic_fields() {
    let output = run_fixture_with_args("unsafe-event", &["--format", "json"]);

    assert_exit_code(&output, 1);

    let json = parse_json_output(&output);

    let diagnostics = json.as_array().expect("JSON output should be an array");

    assert!(!diagnostics.is_empty());

    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic["rule_id"] == "F004")
        .expect("expected F004 diagnostic");

    assert_eq!(diagnostic["rule_id"], "F004");
    assert_eq!(diagnostic["severity"], "WARNING");
    assert_eq!(
        diagnostic["message"],
        "Potentially unsafe network event handler detected."
    );
    assert_eq!(diagnostic["file"], "server.lua");

    assert!(diagnostic["line"].is_number());
    assert!(diagnostic["column"].is_number());

    assert!(diagnostic["explanation"].is_string());
    assert!(diagnostic["suggestion"].is_string());
}

#[test]
fn json_output_contains_multiple_diagnostics() {
    let output = run_fixture_with_args("client-controlled-event", &["--format", "json"]);

    assert_exit_code(&output, 1);

    let json = parse_json_output(&output);

    let diagnostics = json.as_array().expect("JSON output should be an array");

    let rule_ids: Vec<&str> = diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic["rule_id"].as_str())
        .collect();

    assert!(rule_ids.contains(&"F008"));
    assert!(rule_ids.contains(&"F006"));
    assert!(rule_ids.contains(&"F004"));
}

#[test]
fn json_output_respects_warning_severity_filter() {
    let output = run_fixture_with_args(
        "client-controlled-event",
        &["--format", "json", "--severity", "warning"],
    );

    assert_exit_code(&output, 1);

    let json = parse_json_output(&output);

    let diagnostics = json.as_array().expect("JSON output should be an array");

    assert!(!diagnostics.is_empty());

    for diagnostic in diagnostics {
        assert_ne!(
            diagnostic["severity"], "INFO",
            "INFO diagnostic should be filtered from warning output"
        );
    }

    let rule_ids: Vec<&str> = diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic["rule_id"].as_str())
        .collect();

    assert!(rule_ids.contains(&"F004"));
    assert!(rule_ids.contains(&"F008"));
    assert!(!rule_ids.contains(&"F006"));
}

#[test]
fn json_output_with_all_diagnostics_filtered_exits_successfully() {
    let output = run_fixture_with_args(
        "debug-print",
        &["--format", "json", "--severity", "warning"],
    );

    assert_exit_code(&output, 0);

    let json = parse_json_output(&output);

    let diagnostics = json.as_array().expect("JSON output should be an array");

    assert!(
        diagnostics.is_empty(),
        "expected an empty JSON array when all diagnostics are filtered"
    );
}

#[test]
fn missing_cli_arguments_exit_with_code_2() {
    let output = run_cli(&[]);

    assert_exit_code(&output, 2);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
}

#[test]
fn invalid_format_exits_with_code_2() {
    let output = run_cli(&[
        "tests/fixtures/valid-resource",
        "--format",
        "invalid-format",
    ]);

    assert_exit_code(&output, 2);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid value"));
}

#[test]
fn invalid_severity_exits_with_code_2() {
    let output = run_cli(&["tests/fixtures/valid-resource", "--severity", "critical"]);

    assert_exit_code(&output, 2);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid value"));
}

#[test]
fn invalid_manifest_exits_with_code_3() {
    let output = run_fixture("invalid-manifest");

    assert_exit_code(&output, 3);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("failed to analyze"));
    assert!(stderr.contains("failed to parse fxmanifest.lua"));
}

#[test]
fn cli_errors_do_not_emit_diagnostics() {
    let output = run_cli(&["--severity", "critical"]);

    assert_exit_code(&output, 2);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.is_empty());
}

#[test]
fn lua54_no_does_not_report_f003() {
    let output = run_fixture("lua54-no");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("F003"));
    assert!(stdout.contains("No problems found."));
}

#[test]
fn wait_with_nonzero_interval_does_not_report_f005() {
    let output = run_fixture("performance-safe");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("F005"));
    assert!(stdout.contains("No problems found."));
}

#[test]
fn wait_zero_outside_loop_does_not_report_f005() {
    let output = run_fixture("wait-outside-loop");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("F005"));
    assert!(stdout.contains("No problems found."));
}

#[test]
fn print_inside_string_does_not_report_f006() {
    let output = run_fixture("debug-print-safe");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("F006"));
    assert!(stdout.contains("No problems found."));
}

#[test]
fn multiple_events_detects_unsafe_event() {
    let output = run_fixture("multiple-events");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("F004"));
    assert!(stdout.contains("F007"));
}

#[test]
fn ace_permission_check_prevents_f004_and_f007() {
    let output = run_fixture("permission-safe-event");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(!stdout.contains("F004"));
    assert!(!stdout.contains("F007"));
    assert!(stdout.contains("No problems found."));
}

#[test]
fn non_privileged_event_does_not_report_f007() {
    let output = run_fixture("non-privileged-event");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(!stdout.contains("F007"));
}

#[test]
fn mismatched_event_names_do_not_report_f004_or_f007() {
    let output = run_fixture("mismatched-events");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(!stdout.contains("F004"));
    assert!(!stdout.contains("F007"));
    assert!(stdout.contains("No problems found."));
}

#[test]
fn trigger_server_event_with_table_argument_reports_f008() {
    let output = run_fixture("server-event-table-argument");

    assert_exit_code(&output, 1);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("F008"));
    assert!(stdout.contains("Client-controlled value"));
}
