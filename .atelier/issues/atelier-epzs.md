---
created_at: "2026-06-15T15:17:23.703959381+00:00"
id: "atelier-epzs"
issue_type: "task"
labels:
- "deletion"
- "root-crate"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vu2b"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Delete root crate source modules after migration"
updated_at: "2026-06-15T15:17:23.703959381+00:00"
---

## Description

Delete the old root crate source tree after each ownership slice has moved: root lib/main, models, record_store, activity, record_id, workflow_policy, projection_index, db, and commands modules should not remain as compatibility facades.

## Outcome

- Root `src/lib.rs`, `src/main.rs`, `src/models.rs`, `src/record_store.rs`, `src/activity.rs`, `src/record_id.rs`, `src/workflow_policy.rs`, `src/projection_index.rs`, `src/db/`, and `src/commands/` are deleted after their ownership slices move.
- No compatibility aliases, re-export facades, staged deprecations, fallback readers, or old-command shims remain unless a human creates explicit follow-up scope.
- Remaining crate code imports from owning crates instead of root module paths.

## Evidence

- File inventory transcript proves the named root source files and directories are absent.
- Search transcript proves no code imports `atelier::...` root re-export paths, `crate::commands`, `crate::db`, or deleted root module paths.
- `RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets` passes after deletion.
