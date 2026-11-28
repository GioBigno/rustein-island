# Implementation Plan: GitHub Actions CI Workflow

## Goal
Add a GitHub Actions CI workflow that runs three separate jobs (`format`, `clippy`, `test`) on every push to `main` and every pull request targeting `main`.

## Files to Create
- `.github/workflows/ci.yml`:
  - Trigger on `push` to `main` and `pull_request` targeting `main`.
  - Set `permissions: contents: read` and `CARGO_TERM_COLOR: always`.
  - Job `format`: checkout, install stable Rust with `rustfmt`, run `cargo fmt --all -- --check`.
  - Job `clippy`: checkout, install stable Rust with `clippy`, cache Cargo artifacts (`Swatinem/rust-cache@v2`), run `cargo clippy --workspace --all-targets -- -W clippy::pedantic -D warnings`.
  - Job `test`: checkout, install stable Rust, cache Cargo artifacts (`Swatinem/rust-cache@v2`), run `cargo test --workspace`.

## Validation
- Verify YAML syntax of `.github/workflows/ci.yml`.
- Run `cargo fmt --all -- --check` locally (required before committing per `AGENTS.md`).
- Check `git status` and `git diff`.
- Record outcomes in `.agents/plans/plan-2026-11-28-001/result.md`.
- Make an atomic conventional commit (e.g., `ci: add GitHub Actions workflow for format, clippy, and test`).
