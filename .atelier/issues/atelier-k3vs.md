---
created_at: "2026-06-13T21:58:03.424579938+00:00"
id: "atelier-k3vs"
issue_type: "task"
labels:
- "activity"
- "architecture"
- "artifact-update"
- "stabilization"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Define canonical activity sidecar ownership"
updated_at: "2026-06-13T23:58:07.952741567+00:00"
---

## Description

Issue activity sidecars are canonical Markdown, but they are written through a
separate activity API that discovers the repository from the current working
directory. Projection freshness intentionally skips `.activity` directories.
The architecture needs an explicit ownership decision before implementation
normalizes or preserves that split.

## Outcome

- Architecture docs define whether activity sidecars are owned by RecordStore
  or by a separate canonical sidecar API.
- Activity writing, validation, export/rebuild, and projection freshness
  behavior match that documented owner.
- Commands that append activity do not rely on ambiguous current-directory
  discovery when the caller already has a storage layout or canonical path.

## Evidence

- Architecture artifact diff records the activity sidecar owner, freshness
  semantics, and validation responsibility.
- Code review artifact or diff shows activity writes routed through the chosen
  boundary without duplicating canonical Markdown rules.
- Focused history/activity tests prove activity entries survive rebuild and
  malformed activity is handled according to the documented contract.
- `rg` command output for `activity_log`, `.activity`, and
  `skip_sidecar_directory` classifies every retained exception.
- `atelier lint`, `atelier export --check`, and relevant focused tests pass.

## Notes

Audit evidence: `docs/architecture/markdown-first-record-store.md` describes
canonical activity sidecars, `src/commands/activity_log.rs` writes them through
cwd lookup and `src/activity.rs`, while `src/projection_index.rs` skips
`.activity` paths during freshness snapshots.
