# Contributing to fivem-doctor

Thank you for your interest in contributing to `fivem-doctor`.

`fivem-doctor` is an open-source static analyzer and quality checker for FiveM resources. Contributions are welcome, especially bug fixes, analysis improvements, new rules, tests, and documentation improvements.

## Before Contributing

For larger changes, please open an Issue first to discuss the proposal.

Small fixes such as documentation corrections or obvious bug fixes can generally be submitted directly as a Pull Request.

Please keep changes focused and avoid unrelated modifications.

## Development Environment

`fivem-doctor` is written in Rust.

You will need:

- Rust toolchain with Cargo
- Git

Clone the repository and enter the project directory:

```bash
git clone https://github.com/r4gi-dev/fivem-doctor.git
cd fivem-doctor
```

Build the project:

```bash
cargo build
```

## Before Opening a Pull Request

Run the following checks:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

All checks should pass before submitting a Pull Request.

## Adding or Changing Rules

Rules are identified by stable rule IDs such as `F001`, `F002`, and so on.

When adding or changing a rule:

1. Clearly define what the rule detects.
2. Avoid relying on formatting or superficial source-text matching when AST analysis can be used.
3. Consider false positives and false negatives.
4. Add positive tests for cases that should be detected.
5. Add negative tests for cases that should not be detected.
6. Provide a clear diagnostic message.
7. Include an explanation and suggestion when appropriate.
8. Update the project specification when the rule changes the documented behavior.

Security-related rules are heuristic checks. They should not claim to provide complete security verification.

## Tests

Tests are primarily located under:

```text
tests/
```

Fixtures should represent realistic FiveM resource structures where possible.

When fixing a bug, please add a regression test so that the issue does not silently return.

## Code Style

Use standard Rust formatting:

```bash
cargo fmt
```

Prefer small, focused changes.

Avoid introducing unnecessary dependencies or architectural changes unless they are required for the proposed change.

## Pull Requests

A Pull Request should explain:

- What was changed
- Why the change was necessary
- How the change was tested
- Any known limitations or possible false positives

Keep Pull Requests focused on a single purpose whenever possible.

## Issues

When reporting a bug, include:

- `fivem-doctor` version
- Operating system
- Relevant command
- Relevant resource structure or minimal reproduction
- Actual behavior
- Expected behavior
- Error output, if applicable

Please avoid including private server credentials, personal information, or other sensitive data.

## Scope

Please keep contributions aligned with the project's documented scope and roadmap.

Features such as AI-assisted analysis, runtime monitoring, automatic code modification, and full IDE/LSP functionality are outside the current v0.1 scope unless explicitly planned in the project specification.

## License

By contributing to `fivem-doctor`, you agree that your contributions will be licensed under the MIT License included in this repository.