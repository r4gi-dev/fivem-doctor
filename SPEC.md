# fivem-doctor Specification

**Status:** Implemented
**Version:** 0.1.0
**Project Type:** Open Source
**Language:** Rust
**Target:** FiveM Resource Developers

---

## 1. Project Overview

`fivem-doctor` is a static analysis and quality-checking tool for FiveM resources.

The primary goal is to detect common mistakes, configuration problems, potentially unsafe code patterns, and other issues before a resource is deployed to a FiveM server.

The tool is designed as a command-line application and should be usable in:

- Local development
- CI/CD
- GitHub Actions
- Resource review
- Server maintenance
- Open-source FiveM resource development

### Example

```bash
fivem-doctor ./resources/my-resource
```

Example output:

```text
FiveM Doctor v0.1.0

Analyzing: my-resource

Manifest
  笨・fxmanifest.lua found
  笨・Resource manifest is valid

Files
  笨・All referenced files exist

Lua
  笞 Potentially unsafe server event

Security
  笞 Client-controlled value detected

Summary
  Errors:   0
  Warnings: 2
  Info:     1

Analysis completed.
```

---

# 2. Goals

The project has the following primary goals.

## 2.1 Detect common FiveM resource problems

The tool should detect problems that can be identified without executing the resource.

Examples:

- Missing `fxmanifest.lua`
- Missing files referenced by the manifest
- Invalid manifest declarations
- Invalid Lua syntax
- Suspicious network event patterns
- Potential client/server trust issues
- Potentially expensive loops
- Deprecated or unnecessary configuration
- Invalid or suspicious resource dependencies

---

## 2.2 Provide useful diagnostics

Diagnostics should provide enough information for developers to understand and fix the problem.

Each diagnostic should, where possible, include:

- Rule ID
- Severity
- Message
- File
- Line
- Column
- Explanation
- Suggested fix

Example:

```text
WARNING F004

Potentially unsafe server event.

Event:
shop:buy

File:
server/shop.lua:42

The event accepts client-controlled values.

Suggestion:
Validate all values received from the client on the server.
```

---

## 2.3 Be framework-independent

The core analyzer must not depend on a specific FiveM framework.

The core should support:

- Standalone resources
- QBCore
- ESX
- Other frameworks

Framework-specific rules may be added later.

---

## 2.4 Be suitable for CI/CD

The analyzer should eventually support machine-readable output.

Planned formats include:

- Terminal
- JSON
- SARIF

The CLI should be capable of returning a non-zero exit code when configured severity thresholds are exceeded.

---

## 2.5 Be cross-platform

The project should support:

- Windows
- Linux
- macOS

The primary development environment is not restricted to a single operating system.

---

# 3. Non-Goals

The following features are explicitly outside the scope of v0.1.

## 3.1 Runtime monitoring

`fivem-doctor` will not execute FiveM resources.

It will not attempt to:

- Start a FiveM server
- Monitor runtime performance
- Monitor server CPU usage
- Monitor server memory usage
- Inspect live players

These features may be considered in future projects or versions.

---

## 3.2 Automatic code modification

v0.1 will not automatically modify source code.

The analyzer may provide suggestions, but it must not silently rewrite files.

Future versions may provide an explicit:

```bash
fivem-doctor fix
```

command.

---

## 3.3 AI analysis

AI/LLM functionality is not part of the v0.1 core.

Future versions may optionally support:

- Local LLMs
- Cloud LLMs
- Code explanations
- Fix suggestions
- AI-assisted code review

The core analyzer must remain fully functional without an LLM.

---

## 3.4 Full Lua language server

`fivem-doctor` is not intended to become a complete Lua language server.

IDE-specific functionality should remain outside the core analyzer.

---

## 3.5 Full framework implementation

The project will not implement QBCore, ESX, ox_lib, or other frameworks.

Framework support will consist of static-analysis rules and metadata.

---

# 4. Supported Platforms

The initial target platforms are:

| Platform | Support |
|---|---|
| Windows | Required |
| Linux | Required |
| macOS | Required |

The application should not depend on platform-specific functionality unless absolutely necessary.

---

# 5. Technology Stack

## 5.1 Programming Language

Rust.

Reasons:

- Cross-platform compilation
- Strong type safety
- Good CLI ecosystem
- High performance
- Easy distribution as a single binary
- Suitable for static analysis tools

---

## 5.2 Rust Dependencies

The v0.1.0 implementation uses the following dependencies:

- `anyhow` 窶・application-level error handling
- `clap` 窶・CLI argument parsing
- `full_moon` 窶・Lua parsing and AST analysis
- `serde` 窶・diagnostic serialization
- `serde_json` 窶・JSON output
- `thiserror` 窶・typed error support

Additional dependencies may be introduced in future versions when required.

Future candidates include:

- `clap` 窶・CLI argument parsing
- `serde` 窶・serialization
- `serde_json` 窶・JSON output
- `toml` 窶・configuration
- `anyhow` 窶・application-level error handling
- `thiserror` 窶・typed errors
- Lua parser/AST library 窶・Lua analysis
- `globset` or equivalent 窶・manifest glob handling

Dependencies should remain minimal where practical.

---

# 6. Project Architecture

The project should be divided into the following layers:

```text
CLI
 笏・ 笆ｼ
Analyzer
 笏・ 笏懌楳笏 Resource Analyzer
 笏懌楳笏 Manifest Analyzer
 笏披楳笏 Lua Analyzer
 笏・ 笆ｼ
Rules
 笏・ 笏懌楳笏 Manifest Rules
 笏懌楳笏 Security Rules
 笏披楳笏 Performance Rules
 笏・ 笆ｼ
Diagnostics
 笏・ 笆ｼ
Reporters
 笏懌楳笏 Terminal
 笏懌楳笏 JSON
 笏披楳笏 SARIF (future)
```

The analyzer should not directly print output.

Rules should generate structured diagnostics.

Reporters should convert diagnostics into user-facing output.

---

# 7. Repository Structure

The initial repository structure should be:

```text
fivem-doctor/
笏・笏懌楳笏 Cargo.toml
笏懌楳笏 Cargo.lock
笏・笏懌楳笏 README.md
笏懌楳笏 SPEC.md
笏懌楳笏 LICENSE
笏懌楳笏 CONTRIBUTING.md
笏懌楳笏 CODE_OF_CONDUCT.md
笏披楳笏 CHANGELOG.md
笏・笏懌楳笏 .gitignore
笏懌楳笏 rustfmt.toml
笏・笏懌楳笏 .github/
笏・  笏懌楳笏 workflows/
笏・  笏・  笏懌楳笏 ci.yml
笏・  笏・  笏披楳笏 release.yml
笏・  笏・笏・  笏懌楳笏 ISSUE_TEMPLATE/
笏・  笏・  笏懌楳笏 bug_report.yml
笏・  笏・  笏披楳笏 feature_request.yml
笏・  笏・笏・  笏披楳笏 pull_request_template.md
笏・笏懌楳笏 src/
笏・  笏懌楳笏 main.rs
笏・  笏・笏・  笏懌楳笏 cli/
笏・  笏・  笏懌楳笏 mod.rs
笏・  笏・  笏披楳笏 args.rs
笏・  笏・笏・  笏懌楳笏 analyzer/
笏・  笏・  笏懌楳笏 mod.rs
笏・  笏・  笏懌楳笏 resource.rs
笏・  笏・  笏懌楳笏 manifest.rs
笏・  笏・  笏披楳笏 lua.rs
笏・  笏・笏・  笏懌楳笏 rules/
笏・  笏・  笏懌楳笏 mod.rs
笏・  笏・  笏披楳笏 lua.rs
笏・  笏・笏・  笏披楳笏 report/
笏・      笏懌楳笏 mod.rs
笏・      笏懌楳笏 terminal.rs
笏・      笏披楳笏 json.rs
笏・笏披楳笏 tests/
    笏懌楳笏 fixtures/
    笏・  笏懌楳笏 valid-resource/
    笏・  笏懌楳笏 missing-file/
    笏・  笏懌楳笏 unsafe-event/
    笏・  笏披楳笏 performance/
    笏・    笏披楳笏 integration.rs
```

The structure may change during development if implementation experience indicates a better architecture.

---

# 8. CLI Specification

## 8.1 Initial command

The v0.1 CLI should support:

```bash
fivem-doctor <PATH>
```

Example:

```bash
fivem-doctor ./resources/my-resource
```

---

## 8.2 Planned command structure

The following structure is planned for future versions:

```bash
fivem-doctor analyze <PATH>
fivem-doctor init
fivem-doctor rules
fivem-doctor fix
```

However, these commands are not required for v0.1.

---

# 9. CLI Options

The following options are planned.

## Output format

Implemented in v0.1.0:

```bash
--format terminal
--format json
```

Default:

```text
terminal
```

Future:

```text
sarif
```

---

## Severity threshold

Example:

```bash
--severity warning
```

Possible levels:

```text
error
warning
info
```

The semantics are finalized for v0.1.0 as documented in Section 30.

---

## Configuration file

Configuration files are **not implemented in v0.1.0**.

The planned configuration file is:

```text
fivem-doctor.toml
```

Configuration support is planned for a future release and may include rule severity, ignored paths, and framework settings.
---

# 10. Resource Discovery

The analyzer should accept either:

```text
resource directory
```

or, in future versions:

```text
resources directory
```

For v0.1, a single FiveM resource directory is the primary target.

Example:

```text
my-resource/
笏懌楳笏 fxmanifest.lua
笏懌楳笏 client/
笏懌楳笏 server/
笏披楳笏 shared/
```

---

# 11. fxmanifest Analysis

The analyzer must detect:

```text
fxmanifest.lua
```

and parse supported manifest declarations.

The following declarations should be considered during v0.1 development:

```lua
fx_version
game
lua54
client_script
client_scripts
server_script
server_scripts
shared_script
shared_scripts
file
files
dependency
dependencies
export
exports
server_export
server_exports
```

The implementation should follow the official FiveM resource manifest behavior as closely as practical.

---

# 12. Manifest Rule Specification

## F001 窶・Missing Resource Manifest

### Severity

`ERROR`

### Description

No `fxmanifest.lua` was found.

### Example

```text
ERROR F001

fxmanifest.lua was not found.

The target directory does not appear to be a valid FiveM resource.
```

---

## F002 窶・Referenced File Does Not Exist

### Severity

`ERROR`

### Description

A file referenced by the resource manifest does not exist.

### Example

Manifest:

```lua
client_script 'client/main.lua'
```

File:

```text
client/main.lua
```

does not exist.

Diagnostic:

```text
ERROR F002

Referenced file does not exist:

client/main.lua

Location:
fxmanifest.lua:5
```

---

## F003 窶・Deprecated Lua 5.4 Manifest Setting

### Severity

`INFO`

### Description

Detect explicit `lua54` configuration.

The tool should inform the user that Lua 5.4 is the default runtime and the manifest setting is deprecated.

Example:

```text
INFO F003

The `lua54` manifest setting is deprecated.

Lua 5.4 is now the default runtime.
```

This rule must not be treated as an error.

---

# 13. Lua Analysis

The analyzer should parse Lua source code into an AST where practical.

The analyzer should not rely exclusively on regular expressions for structural analysis.

Regular expressions may be used for limited heuristics where AST analysis is unnecessary.

The analyzer should preserve:

- File path
- Line number
- Column number

for diagnostics.

---

# 14. Security Rules

Security analysis is a major long-term feature of `fivem-doctor`.

The v0.1.0 implementation introduces basic security heuristics without attempting to prove that code is secure or insecure.

F004, F007, and F008 are heuristic checks. Their findings indicate patterns that deserve manual review and are not, by themselves, proof of a vulnerability.

---

## F004 窶・Potentially Unsafe Server Event

### Severity

`WARNING`

### Description

Detect server-side network events that accept client-controlled values and may perform sensitive operations.

Potentially sensitive values include:

```text
money
price
amount
item
permission
job
grade
identifier
source
```

Example:

```lua
RegisterNetEvent('shop:buy')

AddEventHandler('shop:buy', function(item, price)
    -- sensitive operation
end)
```

Possible diagnostic:

```text
WARNING F004

Potentially unsafe server event.

Event:
shop:buy

The event receives client-controlled values.

Location:
server/shop.lua:42

Consider validating all client-provided values on the server.
```

The rule should avoid claiming a vulnerability unless sufficient evidence exists.

---

# 15. F005 窶・Potentially Expensive Frame Loop

### Severity

`WARNING`

### Description

Detect potentially expensive loops containing:

```lua
Wait(0)
```

or equivalent frame-level execution.

Example:

```lua
CreateThread(function()
    while true do
        Wait(0)

        PerformExpensiveOperation()
    end
end)
```

Diagnostic:

```text
WARNING F005

Potentially expensive frame loop.

The loop executes every frame using Wait(0).

Location:
client/main.lua:12

Consider whether this code needs to run every frame.
```

`Wait(0)` must not automatically be considered an error.

The rule is intended to identify potentially problematic code for manual review.

---

# 16. F006 窶・Debug Output

### Severity

`INFO`

### Description

Detect common debug output such as:

```lua
print(...)
```

Example:

```text
INFO F006

Debug output detected.

Location:
client/main.lua:52
```

This rule should be configurable and may be disabled by users.

---

# 17. F007 窶・Potential Privileged Event

### Severity

`WARNING` or `ERROR` depending on confidence.

### Description

Detect server network events that appear to perform privileged operations without an obvious authorization check.

Potential indicators include:

```text
admin
give
remove
set
addmoney
removemoney
kick
ban
permission
group
job
```

Example:

```lua
RegisterNetEvent("admin:giveItem")

AddEventHandler("admin:giveItem", function(item, amount)
    -- privileged operation
end)
```

Potential diagnostic:

```text
WARNING F007

Potential privileged network event.

The event name and operations suggest administrative behavior.

No obvious authorization check was detected.

Location:
server/admin.lua:12
```

This is a heuristic.

The analyzer must clearly communicate that the result is not proof of a vulnerability.

---

# 18. F008 窶・Client-Controlled Value

### Severity

`WARNING`

### Description

Detect potentially sensitive values passed directly from client code to server events.

Example:

```lua
TriggerServerEvent(
    "bank:withdraw",
    amount
)
```

Diagnostic:

```text
WARNING F008

Client-controlled value passed to a server event.

Value:
amount

Event:
bank:withdraw

The server should validate this value.
```

---

# 19. Dependency Analysis

The analyzer should inspect dependency declarations.

Example:

```lua
dependency 'ox_lib'
dependency 'qb-core'
```

For v0.1, dependency analysis is limited to manifest declarations.

Future versions may inspect an entire FiveM server resource tree.

---

# 20. Export Analysis

The analyzer should eventually track resource exports.

Example:

```lua
exports('GetPlayer', function()
    ...
end)
```

and references such as:

```lua
exports.myresource:GetPlayer()
```

Future diagnostics may include:

```text
WARNING

Unknown export:
myresource:GetPlayer

No matching export declaration was detected.
```

Full cross-resource export analysis is not required for v0.1.

---

# 21. Diagnostic Model

All rules should produce a common diagnostic structure.

Conceptually:

```text
Diagnostic {
    rule_id
    severity
    message
    file
    line
    column
    explanation
    suggestion
}
```

Example JSON:

```json
{
  "rule": "F004",
  "severity": "warning",
  "message": "Potentially unsafe server event",
  "file": "server/shop.lua",
  "line": 42,
  "column": 1,
  "explanation": "The event accepts client-controlled values.",
  "suggestion": "Validate the received values on the server."
}
```

---

# 22. Severity Levels

The initial severity levels are:

```text
ERROR
WARNING
INFO
```

## ERROR

A definite problem or invalid configuration.

Examples:

- Missing manifest
- Missing referenced file

## WARNING

Potential problem requiring developer review.

Examples:

- Potentially unsafe event
- Potentially expensive loop

## INFO

Informational finding.

Examples:

- Debug output
- Deprecated configuration

---

# 23. Rule ID Convention

Rules use the following format:

```text
Fxxx
```

Examples:

```text
F001
F002
F003
F004
```

Rule IDs should remain stable after release.

If the behavior of a rule changes significantly, the project should document the change in `CHANGELOG.md`.

---

# 24. Configuration

Future configuration will use:

```text
fivem-doctor.toml
```

Example:

```toml
[project]
framework = "qbcore"

[rules]
F001 = "error"
F003 = "info"
F005 = "warning"
F006 = "off"

[ignore]
paths = [
    "vendor/**"
]
```

Framework values may eventually include:

```text
standalone
qbcore
esx
ox
auto
```

The exact configuration schema is not considered stable in v0.1.

---

# 25. Framework Support

The architecture must allow framework-specific rule sets.

Example:

```text
Core Rules
    笏・    笏懌楳笏 FiveM
    笏・    笏披楳笏 Lua

Framework Rules
    笏・    笏懌楳笏 QBCore
    笏懌楳笏 ESX
    笏披楳笏 ox
```

The core analyzer must remain functional when no framework is detected.

---

# 26. Testing

Testing is required for all released rules.

Each rule should have test fixtures.

Example:

```text
tests/
笏披楳笏 fixtures/
    笏懌楳笏 valid-resource/
    笏懌楳笏 missing-file/
    笏懌楳笏 unsafe-event/
    笏披楳笏 performance/
```

Tests should verify:

- Detection
- Non-detection
- Diagnostic rule ID
- Severity
- Location
- Relevant message

False-positive cases should also be tested.

---

# 27. False Positive Policy

False positives are a major concern for static analysis.

`fivem-doctor` must prefer:

> **"Potential problem; review this code."**

over:

> **"This code is definitely vulnerable."**

unless the analyzer has sufficient evidence.

Security rules must therefore use conservative language.

Example:

```text
Potentially unsafe
```

rather than:

```text
VULNERABLE
```

when the analyzer cannot prove the issue.

---

# 28. Performance Requirements

The analyzer should be fast enough to run frequently during development.

Target:

```text
Small resource:
< 1 second
```

The exact benchmark target may be revised after the first implementation.

The tool should avoid unnecessary repeated file parsing.

---

# 29. Output

## Terminal

The default output should be human-readable.

Example:

```text
ERROR F002
server/main.lua:42

Referenced file does not exist:
server/main.lua
```

Colors may be used in terminal output, but output must remain readable when colors are disabled.

---

## JSON

The JSON output should be stable enough for external tooling.

Example:

```bash
fivem-doctor ./resource --format json
```

---

## SARIF

SARIF support is planned for a future release.

This will allow integration with GitHub code scanning and other compatible systems.

---

# 30. Exit Codes

Exit codes for v0.1.0:

| Code | Meaning |
|---:|---|
| `0` | No findings at or above the selected severity threshold |
| `1` | One or more findings at or above the selected severity threshold |
| `2` | CLI or argument/configuration error |
| `3` | Analysis error |

Severity behavior:

- `--severity info`: INFO, WARNING, and ERROR findings are reported.
- `--severity warning`: WARNING and ERROR findings are reported.
- `--severity error`: ERROR findings are reported.

If the filtered diagnostic list is empty, the process exits with `0`. Otherwise it exits with `1`.

---

# 31. GitHub Actions

GitHub Actions integration is planned after the core CLI is stable.

Example future usage:

```yaml
name: FiveM Doctor

on:
  pull_request:

jobs:
  analyze:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - uses: r4gi/fivem-doctor-action@v1
```

The Action should internally invoke the official `fivem-doctor` binary.

The GitHub Action should not contain a separate analyzer implementation.

---

# 32. CI

The project's own CI should verify:

- Rust formatting
- Compilation
- Unit tests
- Integration tests
- Clippy
- Release builds

Planned workflow:

```text
Pull Request
     笏・     笆ｼ
GitHub Actions
     笏・ 笏娯楳笏笏笏ｼ笏笏笏笏笏笏笏笏笏笏笏笏笏笏・ 笆ｼ   笆ｼ             笆ｼ
fmt test          clippy
     笏・     笆ｼ
   build
```

---

# 33. Release Strategy

The project will initially use semantic versioning:

```text
MAJOR.MINOR.PATCH
```

Example:

```text
0.1.0
0.2.0
1.0.0
```

While the project remains below `1.0.0`, APIs and configuration may change.

Stable releases should be published through GitHub Releases.

Future distribution targets may include:

- GitHub Releases
- crates.io
- Homebrew
- Scoop
- Windows binaries
- Linux binaries
- macOS binaries

---

# 34. License

The initial license should be:

```text
MIT License
```

The project should include a complete `LICENSE` file before the first public release.

---

# 35. Contribution Guidelines

Contributors should be able to add new rules without modifying unrelated components.

A new rule should include:

1. Rule implementation
2. Rule ID
3. Documentation
4. Positive test
5. Negative test
6. Example diagnostic

Example:

```text
New Rule
   笏・   笏懌楳笏 Implementation
   笏懌楳笏 Documentation
   笏懌楳笏 Detection test
   笏披楳笏 False-positive test
```

---

# 36. Security Policy

The repository should include:

```text
SECURITY.md
```

Security vulnerabilities in `fivem-doctor` itself should not be disclosed publicly through ordinary GitHub issues when they contain exploitable details.

A dedicated security-reporting process should be established before the first stable release.

---

# 37. Documentation

The documentation should eventually include:

```text
README.md
docs/
笏懌楳笏 getting-started.md
笏懌楳笏 rules/
笏・  笏懌楳笏 F001.md
笏・  笏懌楳笏 F002.md
笏・  笏披楳笏 ...
笏懌楳笏 configuration.md
笏懌楳笏 ci.md
笏披楳笏 contributing.md
```

For v0.1, documentation may remain primarily within:

```text
README.md
SPEC.md
```

---

# 38. Development Roadmap

## v0.1.0

v0.1.0 is the initial implemented release.

### Core

- [x] Rust CLI
- [x] Resource analysis
- [x] `fxmanifest.lua` detection
- [x] Basic manifest parsing
- [x] Referenced-file existence checking
- [x] Lua parsing with `full_moon`
- [x] Structured diagnostic system
- [x] Terminal reporter
- [x] JSON reporter
- [x] Severity filtering
- [x] Exit code handling

### Rules

- [x] F001 窶・Missing Manifest
- [x] F002 窶・Missing Referenced File
- [x] F003 窶・Deprecated `lua54`
- [x] F004 窶・Potentially Unsafe Server Event
- [x] F005 窶・Potentially Expensive Frame Loop
- [x] F006 窶・Debug Output
- [x] F007 窶・Potential Privileged Event
- [x] F008 窶・Client-Controlled Value

### Testing

- [x] Integration tests
- [x] Rule fixtures
- [x] Positive detection tests
- [x] Negative / false-positive tests
- [x] Invalid manifest analysis test
- [x] Multiple-event analysis tests
- [x] Safe-boundary tests

The v0.1.0 test suite currently contains 36 integration tests.

### Repository

- [x] README
- [x] LICENSE
- [x] CONTRIBUTING
- [x] CODE_OF_CONDUCT
- [x] GitHub Actions CI

---

# 39. v0.2.0

Planned features:

- [x] JSON output
- [ ] Configuration file
- [ ] Better Lua analysis
- [ ] More manifest rules
- [ ] Improved diagnostics
- [ ] Ignore rules
- [ ] Rule severity configuration

---

# 40. v0.3.0

Planned features:

- [ ] Security analyzer expansion
- [ ] Network event analysis
- [ ] Dependency analysis
- [ ] Export analysis
- [ ] QBCore rule set
- [ ] Framework auto-detection

---

# 41. v0.4.0

Planned features:

- [ ] GitHub Actions
- [ ] SARIF output
- [ ] GitHub Code Scanning integration
- [ ] Pull Request annotations

---

# 42. v0.5.0

Planned features:

- [ ] ESX rules
- [ ] ox ecosystem rules
- [ ] More advanced framework analysis
- [ ] Rule configuration improvements

---

# 43. v1.0.0

The `1.0.0` release should only occur when:

- CLI behavior is considered stable
- Rule IDs are stable
- Configuration format is stable
- Documentation is sufficient
- Cross-platform builds are reliable
- False-positive rate is acceptable
- CI integration is reliable

---

# 44. Future AI Integration

AI integration is intentionally separated from the core analyzer.

Possible future architecture:

```text
fivem-doctor
      笏・      笆ｼ
Static Analysis
      笏・      笆ｼ
Diagnostics
      笏・      笆ｼ
Optional AI Layer
      笏・ 笏娯楳笏笏笏笏ｴ笏笏笏笏笏・ 笆ｼ         笆ｼ
Explain   Suggest Fix
```

Possible providers:

- Local Ollama
- llama.cpp
- OpenAI-compatible APIs
- Other local inference servers

AI functionality must be optional.

The tool must remain fully functional without Internet access or an external AI provider.

---

# 45. Future Developer Experience

Possible future integrations:

```text
fivem-doctor
      笏・ 笏娯楳笏笏笏笏ｼ笏笏笏笏笏笏笏笏笏笏笏笏・ 笆ｼ    笆ｼ           笆ｼ
CLI  GitHub      VS Code
     Actions
      笏・      笆ｼ
    Cloud
```

Potential future services:

- Web dashboard
- Private repository analysis
- Team reports
- Historical reports
- AI code review
- Server monitoring
- Organization-wide rule configuration

These services are not part of the core OSS project at this stage.

---

# 46. OSS Philosophy

The project should follow these principles.

### 1. Core functionality remains open source

The basic analyzer should remain freely available.

### 2. No mandatory cloud dependency

The CLI must work locally.

### 3. No mandatory AI dependency

AI should never be required to analyze a resource.

### 4. Developer-first design

The tool should prioritize useful diagnostics over maximizing the number of detected issues.

### 5. Conservative security warnings

The analyzer should avoid presenting uncertain findings as confirmed vulnerabilities.

### 6. Small and composable rules

Each rule should have a clear purpose.

### 7. Community-driven development

New rules should be easy for contributors to implement.

---

# 47. Initial Success Criteria

The first version will be considered successful if a developer can:

```text
1. Download/install fivem-doctor
2. Point it at a FiveM resource
3. Receive useful diagnostics
4. Understand why each diagnostic occurred
5. Fix the problem
6. Run the analyzer again
7. Get a clean result
```

Example:

```bash
fivem-doctor ./my-resource
```

竊・
```text
Found 3 warnings.

F002  Missing file
F004  Unsafe event pattern
F005  Potentially expensive loop
```

After fixing:

```bash
fivem-doctor ./my-resource
```

竊・
```text
No problems found.

Analysis completed in 0.42s.
```

---

# 48. Design Principle

The central design principle of `fivem-doctor` is:

> **Find problems before they reach the server.**

The project should not attempt to replace FiveM development tools.

It should provide a lightweight layer of automated review between writing code and deploying a resource.

```text
Write Code
    笏・    笆ｼ
fivem-doctor
    笏・    笏懌楳笏 Find Problems
    笏懌楳笏 Explain Problems
    笏披楳笏 Suggest Improvements
    笏・    笆ｼ
Review
    笏・    笆ｼ
Deploy
```

---

# 49. Specification Status

This document describes the implemented architecture and scope of `fivem-doctor` v0.1.0, together with planned future features.

Implementation details may change in future releases. Significant changes should be documented in `CHANGELOG.md`.

Any significant architectural change should be documented in:

```text
CHANGELOG.md
```

and, where appropriate, reflected in this specification.

**Current status: Implemented**
