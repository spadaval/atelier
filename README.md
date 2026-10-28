# Atelier

Atelier is a Rust CLI for issue tracking and agent-oriented development
workflows.

## Install

On Linux (x86_64 or aarch64, glibc 2.35 or newer) or macOS, install or upgrade
to the latest release with:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh
```

The installer puts `atelier` in `$CARGO_HOME/bin` (`~/.cargo/bin` by default)
and, if that directory is not already on your `PATH`, adds it in your shell
profile; open a new shell to use it. Rerun it to upgrade. Releases, with archives and checksums for each platform,
are on [GitHub](https://github.com/spadaval/atelier/releases). To build from a
source checkout instead, run `cargo install --path crates/atelier-cli --locked`.

Fresh setup, required tools, validation commands, local environment policy, and
the current devcontainer decision live in
[docs/product/development-setup.md](docs/product/development-setup.md).

Repository operating rules for agents live in [AGENTS.md](AGENTS.md).
