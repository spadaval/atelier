# Development Setup

Normal development needs Git and the toolchain pinned in
[rust-toolchain.toml](../../rust-toolchain.toml), including rustfmt and clippy.
Install the default test runner and dependency checker when needed:

```bash
cargo install cargo-nextest --locked
cargo install cargo-machete --locked
```

From the repository root:

```bash
cargo build -p atelier-cli
cargo fmt -- --check
cargo nextest run
target/debug/atelier check
git diff --check
```

The default test run excludes injected failure hooks. Run
`cargo nextest run --features test-hooks` when checking rollback, cleanup
failure, and concurrency paths. Normal release builds contain no fault-injection
environment hooks.

Use `target/debug/atelier` to validate repository CLI changes. The installed
`atelier` (`cargo install --path crates/atelier-cli`) runs this repository's
Claude Code hooks; it can lag `master`, so reinstall after merging changes to
session commands. No external service, provider credential, `.env`, or
devcontainer is required.

Optional `ATELIER_LOG` and `ATELIER_LOG_FORMAT` configure diagnostics. Producer
identity is self-declared; commands with attribution accept an explicit identity
where documented by help. Do not commit credentials or local runtime files.

The ignored `.atelier/runtime/` contains the mutation lock. Commands read
tracked records directly. There is no database or cache to initialize or repair.
`cargo udeps` is an optional nightly cross-check, not a required setup input.

See [standards](../architecture/quality/standards.md) for change-specific proof
and review requirements.
