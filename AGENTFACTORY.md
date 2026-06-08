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
- Tracker replacement MVP:
  `docs/spec/agent-factory/tracker-replacement-mvp.md`

## Tracker

- Tracker: Atelier
- Durable tracker state: committed `.atelier-state/`
- Runtime tracker database: local `.atelier/state.db`, rebuilt from
  `.atelier-state/`
- Archived Beads fallback: `.beads/issues.manual.jsonl` and `.beads/`
  metadata are retained read-only for recovery and audit only.
- Normal tracker commands:
  - `atelier issue ready`
  - `atelier issue list --status open`
  - `atelier issue show <id>`
  - `atelier issue update <id> --claim`
  - `atelier issue update <id> --append-notes "..."`
  - `atelier issue close <id> --reason "..."`
- Sync and state commands:
  - `git pull`
  - `atelier rebuild`
  - `atelier export --check`
  - `atelier export`
  - `git status --short --branch`
  - `git push`
- Tracker health commands:
  - `atelier lint`
  - `atelier lint <id>`
  - `atelier doctor`
- Beads commands are not part of the normal Agent Factory path in this
  repository. Use `bd` only for explicit archival bookkeeping while retiring the
  old tracker, and never use interactive commands such as `bd edit`.

## Checks

- Markdown formatting: `git diff --check -- '*.md'`
- Rust formatting: `cargo fmt -- --check`
- Diff whitespace: `git diff --check`
- Tracker export freshness: `atelier export --check`
- Tracker lint: `atelier lint`
- Tracker health: `atelier doctor`
- Full repository check: `cargo test`

## Product-Specific Skills

- None yet. Add repository-local skills under `.agents/skills/` and bind them
  here when repeated Atelier-specific workflows emerge.
