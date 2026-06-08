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
- Treat `.atelier-state/` as the committed durable tracker state and
  `.atelier/state.db` as local runtime state that can be rebuilt.
- `.beads/` is archived read-only recovery data. Do not use `bd` for normal
  planning, execution, or handoff in this repository.

## Repository Shape

- Product intent: `SPEC.md`
- Domain language: `CONTEXT.md`
- Documentation map: `docs/index.md`
- Architecture: `docs/architecture/index.md`
- Quality and validation: `docs/architecture/quality/`

## Common Commands

```bash
cargo fmt -- --check
cargo test
git diff --check
atelier issue ready
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
