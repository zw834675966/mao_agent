---
name: fast-ci
description: Executes fast, local multi-gate verification matching GitHub Actions CI (.github/workflows/ci.yml) including rustfmt checks, clippy linting (-D warnings), and test suite execution without default features. Use before committing, creating pull requests, or verifying local code changes.
---

# Fast CI Verification Skill

Provides a standardized, multi-gate local verification workflow that mirrors the repository's GitHub Actions CI (`.github/workflows/ci.yml`). It validates formatting, enforces strict zero-warning linting, and runs unit and integration test suites in lightweight mode (`--no-default-features`).

## When to Use

- Before committing changes or creating pull requests.
- After code modifications, refactorings, or dependency changes.
- To diagnose CI failures locally without pushing commits.

---

## Execution Steps

Run the following three verification gates in sequence from the project root (`D:/rust/mao_agent`):

### 1. Style Check (`rustfmt`)
Validates that all code conforms to Rust formatting standards.

```bash
cargo fmt --check
```

- **If this gate fails**: Formatting differences are present. Run `cargo fmt` to automatically format all files, then re-run `cargo fmt --check`.

### 2. Linter Gate (`clippy`)
Enforces strict compiler linter checks with warnings treated as hard errors across all targets (bin, lib, tests, benches) under `--no-default-features`.

```bash
cargo clippy --no-default-features --all-targets -- -D warnings
```

- **If this gate fails**: Clippy emitted warnings or errors. Fix every warning; warnings are treated as build blockers per repository rules.

### 3. Automated Test Suite (`cargo test`)
Runs all unit tests, doc tests, and integration tests under `--no-default-features` (skips the heavy ONNX/FastEmbed dependency tree).

```bash
cargo test --no-default-features
```

- **If this gate fails**: Inspect the test failure stack trace, locate failing assertions in `tests/` or unit tests, and resolve before proceeding.

---

## Validation Criteria

1. **Exit Codes**: All three commands must return exit code `0`.
2. **Zero Warnings**: No compiler or clippy warnings are permitted (`-D warnings`).
3. **No Network Dependency**: The verification must complete entirely offline with `--no-default-features`.
