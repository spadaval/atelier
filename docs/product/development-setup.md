# Development Setup

Normal development needs Git and the toolchain pinned in
[rust-toolchain.toml](../../rust-toolchain.toml), including rustfmt and clippy.
A release build or `cargo install` also needs Node.js and pnpm, which build the
web app `atelier serve` embeds; debug builds and the Rust tests do not
([site](../architecture/site.md#building-and-shipping-it)).
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

Day to day, run formatting and the tests that cover your change
(`cargo nextest run -p <crate>` or a filter). The full suites, with and without
`test-hooks`, run on Linux in CI on the pull request, and that run gates the
merge (see [AGENTS.md](../../AGENTS.md)). CI does not run on macOS, which is
supported on a best-effort basis (see [README.md](../../README.md#platform-support)).

Tests run with `ATELIER_NO_UPDATE_CHECK=1`, which `.cargo/config.toml` sets,
so `doctor` and `status` stay off the network (see the
[update check](sessions-and-hooks.md#update-check)).

The default test run excludes injected failure hooks. Run
`cargo nextest run --features test-hooks` when checking rollback, cleanup
failure, and concurrency paths. Normal release builds contain no fault-injection
environment hooks.

Use `target/debug/atelier` to validate repository CLI changes. The installed
`atelier` runs this repository's Claude Code hooks. Here, install it from the
checkout with `cargo install --path crates/atelier-cli --locked`; it can lag
`master`, so reinstall after merging changes to session commands. Elsewhere,
the release installer in the [README](../../README.md#install) installs and
upgrades it. No external service, provider credential, `.env`, or
devcontainer is required.

Atelier reads no logging configuration; the former `--log-level`,
`--log-format`, `ATELIER_LOG` and `ATELIER_LOG_FORMAT` were removed. Producer
identity is self-declared; commands with attribution accept an explicit identity
where documented by help. Do not commit credentials or local runtime files.

The ignored `.atelier/runtime/` contains the mutation lock. Commands read
tracked records directly. There is no record database or cache to initialize
or repair.
`cargo udeps` is an optional nightly cross-check, not a required setup input.

See [standards](../architecture/quality/standards.md) for change-specific proof
and review requirements.
