# Atelier

Atelier tracks the work of coding agents in the repository they change. Its
records are Markdown in Git, on the branch that does the work, and `atelier
status` tells each agent thread what it holds and the next command. Every
non-trivial change is reviewed by a thread that did not write it before
`atelier merge` takes it to the base.

## Install

On Linux (x86_64 or aarch64), install or upgrade to the latest release with:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh
```

The installer puts `atelier` in `$CARGO_HOME/bin` (`~/.cargo/bin` by default)
and, if that directory is not already on your `PATH`, adds it in your shell
profile; open a new shell to use it. Rerun it to upgrade. Releases, with archives and checksums for each platform,
are on [GitHub](https://github.com/spadaval/atelier/releases). To build from a
source checkout instead, run `cargo install --path crates/atelier-cli --locked`.

## Platform support

Linux is the supported platform: every change is tested on it in CI, and
releases include Linux binaries only. macOS and Windows are not supported.
Building from source on macOS may work, but nothing tests it.

## Use

What Atelier does is in the [product specification](docs/product.md), and
how it is built in the [design record](docs/design.md). The human's tool, the
Desk, is a separate server and web app in this repository
([desk/](desk/README.md)).

Agents learn Atelier's workflow from the Agent Factory skill, which lives in
[skills/agent-factory](skills/agent-factory). Install it globally with Vercel's
`skills` CLI:

```bash
npx skills add spadaval/atelier --skill agent-factory -g
```

The skill only acts in repositories tracked with Atelier.

Repository operating rules for agents live in [AGENTS.md](AGENTS.md).
