# Agent Instructions

## Start Here

- Load `AGENTFACTORY.md` before coordinated agent work.
- Use the `agent-factory` skill for planning, orchestration, implementation,
  review, validation, documentation, audit, and readiness work.
- Use Atelier (`atelier`) for durable task tracking. Do not replace it with private
  chat notes or ad hoc TODO files.
- Orchestrators assign exactly one agent-factory subskill to each subagent.
- Do not use interactive tracker commands; use explicit `atelier issue`
  commands instead.
- Do not plan or validate work by parsing command-result JSON. Use the focused
  human output from show/list/ready/workflow commands, quiet acknowledgements
  only where the command is naturally terse, and explicit drill-down commands
  when more state is needed.
- Treat `.atelier/` canonical Markdown and tracked config as the committed
  durable tracker state. Treat `.atelier/state.db`, `.atelier/runtime/`,
  identity, locks, diagnostics, and `.atelier/cache/` as ignored local
  runtime/cache state that can be rebuilt or recreated.
- Beads is no longer repository tracking state. Do not use the predecessor CLI
  for planning, execution, handoff, or recovery in this repository.

## Repository Shape

- Product intent: `SPEC.md`
- Domain language: `CONTEXT.md`
- Documentation map: `docs/index.md`
- Architecture: `docs/architecture/index.md`
- Quality and validation: `docs/architecture/quality/`

## Common Commands

Prefer the installed `atelier` command for normal tracker work. Use
`target/debug/atelier` only when testing local CLI changes that have not been
installed yet. Recompile first when that binary is missing or stale relative to
Rust sources/manifests; use `cargo run -- ...` only when a one-off rebuild plus
execution is specifically useful.

```bash
cargo fmt -- --check
cargo nextest run
cargo nextest run --profile extended --run-ignored=only
git diff --check
atelier issue list --ready
atelier export --check
atelier lint
atelier doctor
```

## Current Product State

Atelier is being built from the Chainlink Rust CLI codebase. Milestone 1 has
renamed the primary package, binary, runtime directory, resources, and
user-facing command surface to Atelier while preserving useful inherited
behavior. Treat `SPEC.md` as the target product intent and current code as the
inherited implementation baseline until follow-up migration issues say otherwise.
