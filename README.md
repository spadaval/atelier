# Atelier

Atelier is a Rust CLI for issue tracking and agent-oriented development
workflows.

## Install

On Linux (x86_64 or aarch64), or macOS on a
[best-effort basis](#platform-support), install or upgrade to the latest
release with:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh
```

The installer puts `atelier` in `$CARGO_HOME/bin` (`~/.cargo/bin` by default)
and, if that directory is not already on your `PATH`, adds it in your shell
profile; open a new shell to use it. Rerun it to upgrade. Releases, with archives and checksums for each platform,
are on [GitHub](https://github.com/spadaval/atelier/releases). To build from a
source checkout instead, install Node.js and pnpm, which build the web app
`atelier serve` embeds, and run `cargo install --path crates/atelier-cli --locked`.

## Platform support

Linux is the supported platform: every change is tested on it in CI. macOS
support is unofficial and best effort. Releases still include macOS
binaries, and fixes for macOS bugs are welcome, but CI does not test on
macOS, so a change can break it unnoticed. Windows is not supported.

## Use

`atelier serve` shows every mission's brief, debrief and strategy as pages,
for every Atelier repository under `~/projects`, at `http://127.0.0.1:7341`.
[docs/architecture/site.md](docs/architecture/site.md) says how to develop,
test and run it as a service.

Agents learn Atelier's workflow from the Agent Factory skill, which lives in
[skills/agent-factory](skills/agent-factory). Install it globally with Vercel's
`skills` CLI:

```bash
npx skills add spadaval/atelier --skill agent-factory -g
```

The skill only acts in repositories tracked with Atelier.

Fresh setup, required tools, validation commands, local environment policy, and
the current devcontainer decision live in
[docs/product/development-setup.md](docs/product/development-setup.md).

Repository operating rules for agents live in [AGENTS.md](AGENTS.md).
