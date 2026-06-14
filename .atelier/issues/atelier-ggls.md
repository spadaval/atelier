---
created_at: "2026-06-13T23:10:29.550860996+00:00"
id: "atelier-ggls"
issue_type: "task"
labels:
- "architecture"
- "assignee:root"
- "refactor"
- "stabilization"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T00:16:49.755002804+00:00"
status: "done"
title: "Split runtime database schema and projection migration hotspots"
updated_at: "2026-06-14T00:16:49.755002804+00:00"
---

## Description

Runtime database initialization, issue-ID migration, and projection rebuild
loading still contain large procedural functions surfaced by the large-function
scan. Extract cohesive schema, migration, and projection helpers without
changing supported CLI behavior.

## Outcome

- `src/db/mod.rs` `init_schema` and `migrate_issue_ids_to_text` are split into
  named schema or migration units with clear ownership.
- `src/commands/rebuild.rs` `load_projection` is split or justified behind a
  documented projection-loading boundary.
- Projection freshness, rebuild, and migration behavior remain unchanged except
  for documented bug fixes.

## Evidence

- Code review artifact or diff maps each retained helper to schema, migration,
  or projection ownership.
- Focused rebuild, export, and migration-related test transcript passes.
- Command transcripts for `cargo fmt -- --check`, `target/debug/atelier lint`,
  and `target/debug/atelier export --check` pass.
