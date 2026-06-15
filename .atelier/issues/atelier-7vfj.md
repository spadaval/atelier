---
created_at: "2026-06-15T05:13:54.680676454+00:00"
id: "atelier-7vfj"
issue_type: "task"
labels:
- "fuzz"
- "tests"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Retarget fuzz harnesses to new internal crate APIs"
updated_at: "2026-06-15T05:13:54.680676454+00:00"
---

## Description

Update fuzz harnesses to target the new internal crate APIs after the workspace split.

## Outcome

- Fuzz targets no longer depend on the old single-crate `atelier::db::Database` path.
- Fuzz inputs exercise the new domain, record, workflow, SQLite, and CLI-output boundaries where practical.
- Fuzz build configuration references the workspace crates correctly.

## Evidence

- Fuzz build transcript or focused cargo-fuzz check proves targets compile.
- Search transcript shows old `atelier::db::Database` fuzz imports were removed.
- Evidence record for any deferred fuzz runtime names residual risk and follow-up owner.
