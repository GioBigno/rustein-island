# Result: GitHub Actions CI Workflow

## Changes Made
- Created `.github/workflows/ci.yml` configured to trigger on every `push` to `main` and every `pull_request` targeting `main`, with `permissions: contents: read` and three jobs on `ubuntu-latest`:
  - `format`: runs `cargo fmt --all -- --check`.
  - `clippy`: caches Cargo artifacts with `Swatinem/rust-cache@v2` and runs `cargo clippy --workspace --all-targets -- -W clippy::pedantic -D warnings`.
  - `test`: caches Cargo artifacts with `Swatinem/rust-cache@v2` and runs `cargo test --workspace`.

## Validation Performed
- Parsed `.github/workflows/ci.yml` with `yaml.safe_load` (`YAML OK, jobs: ['format', 'clippy', 'test']`).
- Verified `cargo fmt --all -- --check` exits with code 0.

## Questions and Answers
No clarifying questions were needed for this task.
