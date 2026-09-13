# Changelog

All notable changes to `fivem-doctor` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows Semantic Versioning.

## [0.1.0] - 2026-09-13

### Added

#### Core Analyzer

- Initial Rust CLI implementation.
- FiveM resource analysis from a specified resource directory.
- `fxmanifest.lua` detection.
- Basic `fxmanifest.lua` parsing.
- Referenced-file existence checking.
- Lua source parsing using `full_moon`.
- Structured diagnostic system with:
  - Rule ID
  - Severity
  - Message
  - File
  - Line
  - Column
  - Explanation
  - Suggestion

#### CLI

- Added resource path argument:
  ```text
  fivem-doctor <PATH>
  ```
- Added `--format` option:
  - `terminal`
  - `json`
- Added `--severity` option:
  - `info`
  - `warning`
  - `error`
- Added `--help`.
- Added `--version`.

#### Output

- Added human-readable terminal output.
- Added JSON output.
- Added severity-based diagnostic filtering.

#### Rules

- **F001 — Missing Resource Manifest**
  - Detects resources without `fxmanifest.lua`.

- **F002 — Referenced File Does Not Exist**
  - Detects files referenced by the manifest that do not exist.

- **F003 — Deprecated Lua 5.4 Manifest Setting**
  - Detects explicit `lua54 'yes'` configuration.

- **F004 — Potentially Unsafe Server Event**
  - Detects potentially unsafe server network-event patterns.
  - Uses conservative heuristic analysis.

- **F005 — Potentially Expensive Frame Loop**
  - Detects `Wait(0)` inside loops.
  - Intended to identify code that may execute every frame.

- **F006 — Debug Output**
  - Detects `print(...)` calls.

- **F007 — Potential Privileged Event**
  - Detects potentially privileged network events without an obvious authorization check.
  - Uses conservative heuristic analysis.

- **F008 — Client-Controlled Value**
  - Detects values passed from client code to server events.
  - Intended to identify values that should be validated server-side.

#### Testing

- Added integration-test suite.
- Added rule-specific test fixtures.
- Added positive detection tests.
- Added negative / false-positive tests.
- Added invalid manifest tests.
- Added multiple-event analysis tests.
- Added safe-boundary tests.
- Added tests for severity filtering and diagnostic behavior.

### Changed

- Established a framework-independent analysis architecture.
- Separated analysis, rules, diagnostics, and reporting responsibilities.
- Adopted AST-based Lua analysis through `full_moon` instead of relying exclusively on regular expressions.
- Established stable `Fxxx` rule identifiers.
- Established severity levels:
  - `ERROR`
  - `WARNING`
  - `INFO`
- Established v0.1.0 exit-code behavior:
  - `0` — No findings at or above the selected severity.
  - `1` — Findings detected at or above the selected severity.
  - `2` — CLI or argument/configuration error.
  - `3` — Analysis error.

### Documentation

- Added project README.
- Added project specification (`SPEC.md`).
- Documented v0.1.0 rules and CLI behavior.
- Documented the project's conservative false-positive policy.
- Documented future roadmap and planned functionality.

### Security

- Added initial static security heuristics for network events.
- Security findings are intentionally described as potential problems rather than confirmed vulnerabilities.
- F004, F007, and F008 require developer review and do not constitute proof that a resource is vulnerable.

### Notes

`v0.1.0` is the initial release of `fivem-doctor`.

The following functionality is intentionally not part of v0.1.0:

- Configuration files
- SARIF output
- GitHub Actions integration
- Framework-specific rule sets
- Automatic code modification
- Runtime monitoring
- AI/LLM analysis
- Full Lua language-server functionality
- Cross-resource dependency/export analysis

These features may be considered for future releases.

[0.1.0]: https://github.com/r4gi-dev/fivem-doctor/releases/tag/v0.1.0