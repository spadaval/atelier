---
created_at: "2026-06-23T16:21:20.072882343+00:00"
id: "atelier-u327"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-01T05:58:31.112123203+00:00"
status: "done"
title: "Define persistence migration and hard-removal plan"
updated_at: "2026-07-01T05:58:31.112123203+00:00"
---

## Description

Define the migration strategy for sweeping storage/cache changes in this experimental project, including what can be hard removed.

## Outcome

- The migration plan explicitly permits hard removal of obsolete projection, canonical, and generic-record cache paths.

## Evidence

File changes in `docs/architecture/sqlite-runtime-schema.md` and ADR 0017
explicitly permit hard removal of obsolete generic cache paths, compatibility
migrations, and SQLite-first mutation paths.
