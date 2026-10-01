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

Use `target/debug/atelier` to validate repository CLI changes. The installed
`atelier` can still contain the old model during the rebuild; update it only
when the new model is stable. No external service, provider credential, `.env`,
or devcontainer is required.

Optional `ATELIER_LOG` and `ATELIER_LOG_FORMAT` configure diagnostics. Producer
identity is self-declared; commands with attribution accept an explicit identity
where documented by help. Do not commit credentials or local runtime files.

The ignored `.atelier/runtime/` contains the mutation lock. Commands read
tracked records directly. There is no database or cache to initialize or repair.
`cargo udeps` is an optional nightly cross-check, not a required setup input.

See [standards](../architecture/quality/standards.md) for change-specific proof
and review requirements.
