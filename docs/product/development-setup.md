# Development Setup

This repository should be usable from committed files alone. Normal development
does not require a local `.env` file, secret material, Docker services, or a
committed devcontainer.

## Required Tools

- Git
- Rust `1.95.0` from [rust-toolchain.toml](../../rust-toolchain.toml)
- Rust components: `rustfmt`, `clippy`
- `cargo-nextest` for the default Rust test workflow
- `cargo-machete` for the repo-supported unused dependency scan when
  dependency manifests change

Install cargo subcommands once with:

```bash
cargo install cargo-nextest --locked
cargo install cargo-machete --locked
```

## Fresh Checkout

From the repository root:

```bash
cargo fmt -- --check
cargo nextest run
atelier check
```

Use the installed `atelier` command for normal tracker work. Use
`target/debug/atelier` only when validating local CLI changes before they are
installed.

## Environment And Secrets

No local `.env` or `.env.example` file is required for normal development.
Common repository work should succeed without setting any environment variables.

Optional environment variables are limited to local behavior overrides:

- `ATELIER_LOG` and `ATELIER_LOG_FORMAT` adjust CLI diagnostic logging.

These variables are optional local overrides, not required setup inputs.

`~/.config/atelier.toml` is required only for commands that mutate the
configured provider review artifact. Store the Forgejo admin token there as
`review.providers.forgejo.admin_token`. The token value is local secret
material and must not be committed.

## Local Runtime State

Atelier keeps only the canonical mutation lock under the ignored
`.atelier/runtime/` directory. There is no cache: every command parses the
tracked record files directly. Do not edit ignored runtime files as a
substitute for updating tracked Markdown records, `.atelier/config.toml`, or
`.atelier/workflow.yaml`.

`cargo udeps` is not part of required setup. It remains an advisory
nightly-toolchain cross-check for deeper dependency review when a reviewer asks
for it.

## Devcontainer Decision

No `.devcontainer/` is committed today. That is an explicit defer decision, not
an omission.

Rationale:

- the repository is a single Rust CLI with a pinned toolchain and no required
  service stack;
- the committed setup path already fits in a few host commands;
- a devcontainer would add a second environment contract to maintain before the
  repo has cross-platform or service-topology pressure that justifies it.

Revisit a committed devcontainer when Atelier requires nontrivial external
services, more native tool dependencies, or repeated onboarding failures that
the host-toolchain path does not resolve.
