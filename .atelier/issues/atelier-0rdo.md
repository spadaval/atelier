---
created_at: "2026-06-15T05:13:23.806138213+00:00"
id: "atelier-0rdo"
issue_type: "task"
labels:
- "architecture"
- "docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4ra1"
  - kind: "issue"
    id: "atelier-f74g"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T05:39:37.749219834+00:00"
status: "done"
title: "Define target workspace and crate boundary contract"
updated_at: "2026-06-15T05:39:37.749219834+00:00"
---

## Description

Update the architecture contract for the target Cargo workspace and crate boundaries before implementation begins. This issue defines the destination shape, not the mechanical migration steps.

## Outcome

- Architecture docs define `atelier-core`, `atelier-workflow`, `atelier-records`, `atelier-sqlite`, `atelier-app`, and `atelier-cli` responsibilities.
- Docs define pragmatic layering: intended dependency direction is clear, and temporary adapters are allowed only when tracked by removal work.
- Docs state Rust crate APIs are internal and tests/fuzz targets should move to new internal APIs.
- Docs no longer describe the current single-crate source layout as the target architecture.

## Evidence

- File review shows updated architecture/source-layout documentation and related index references.
- Search transcript shows no stale guidance that tells agents not to look for `crates/` as the target state.
- `atelier lint atelier-0rdo` and `atelier export --check` pass.
