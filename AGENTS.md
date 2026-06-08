# Agent Instructions

## Start Here

- Load `AGENTFACTORY.md` before coordinated agent work.
- Use the `agent-factory` skill for planning, orchestration, implementation,
  review, validation, documentation, audit, and readiness work.
- Use Beads (`bd`) for durable task tracking. Do not replace it with private
  chat notes or ad hoc TODO files.
- Orchestrators assign exactly one agent-factory subskill to each subagent.
- Do not use interactive tracker commands such as `bd edit`; use explicit
  `bd update` flags instead.

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
bd ready
bd lint
bd dolt status
```

## Current Product State

Atelier is being built from the Chainlink Rust CLI codebase. Existing code,
tests, resource paths, and binary names still use Chainlink in many places.
Treat `SPEC.md` as the target product intent and current code as the inherited
implementation baseline until rename or migration beads say otherwise.
