# Agent Factory Binding

This file binds the generic agent-factory operating model to this repository's
concrete files, commands, and product-specific skills.

## Sources

- Agent instructions: `AGENTS.md`
- Docs map: `docs/index.md`
- Domain context: `CONTEXT.md`
- Product intent: `SPEC.md`
- ADR directory: `docs/adr/`
- Architecture index: `docs/architecture/index.md`
- Quality index: `docs/architecture/quality/index.md`
- Architecture quality vocabulary: `docs/architecture/quality/architecture-quality.md`
- Code standards: `docs/architecture/quality/standards.md`
- Validation router: `docs/architecture/quality/validation.md`

## Tracker

- Tracker: Beads
- Tracker database: shared-server Dolt database `atelier`
- Manual tracker backup/export: `.beads/issues.manual.jsonl`
- Do not keep `.beads/issues.jsonl` in this repo. In the current Beads version,
  that path is treated as a startup import source and can cause repeated
  JSONL-to-Dolt imports before mutations.
- Sync commands:
  - `bd dolt pull`
  - `bd dolt push`
  - `bd dolt status`
  - `bd export -o .beads/issues.manual.jsonl`

## Checks

- Markdown formatting: `git diff --check -- '*.md'`
- Rust formatting: `cargo fmt -- --check`
- Diff whitespace: `git diff --check`
- Bead lint: `bd lint`
- Full repository check: `cargo test`

## Product-Specific Skills

- None yet. Add repository-local skills under `.agents/skills/` and bind them
  here when repeated Atelier-specific workflows emerge.
