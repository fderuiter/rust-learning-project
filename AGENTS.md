# AGENTS

Guidelines for contributors working in this Rust and WebAssembly project. These
instructions ensure consistent engineering practices and mirror the checks run
by GitHub Actions so issues are caught before opening a pull request.

## Workflow
- Use a pull‑request based workflow. Keep commits small and focused with clear
  messages.
- Update `CHANGELOG.md` under **Unreleased** for every user‑facing change.
- Keep documentation up to date (e.g. `README.md`, `docs/`).
- Use `rg` for searching and avoid `ls -R` or `grep -R`.

## Code style
- Format Rust code with `cargo fmt --all` before committing.
- Treat all Clippy warnings as errors.
- Document public items with Rustdoc comments.

## Required checks (parity with GitHub Actions)
Run the following commands locally and include their results in the pull
request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --workspace -- -D warnings
cargo test --all-features --workspace
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items --all-features --workspace --examples
```

## Git hygiene
- Only commit files relevant to your change and ensure the worktree is clean
  before committing.
- Do not bypass required checks or CI failures.

Following these practices keeps the codebase healthy and contributions
maintainable.

