---
created_at: "2026-06-10T16:00:59.329418067+00:00"
id: "atelier-yvk6"
issue_type: "task"
labels:
- "activity"
- "migration"
- "script"
- "sqlite"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-krhk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T17:37:47.184466923+00:00"
status: "done"
title: "Add SQLite comments-to-activity migration script"
updated_at: "2026-06-10T17:37:47.184466923+00:00"
---

## Description

Provide the one-off migration path for existing local SQLite comments and close reasons.

What:
- Add `scripts/migrate_sqlite_comments_to_activity.py` as an explicit migration script, not a normal Atelier command.
- Read existing `.atelier/state.db` comments and close reasons.
- Write canonical issue activity sidecar files.
- Support `--dry-run`.
- Refuse to overwrite existing activity IDs.
- Print a conversion summary.

Out of scope:
- Migrating non-comment/non-close-reason runtime data.
- Adding a first-class `atelier migrate` command.

## Outcome

- Running against a temp repo with SQLite comments and close reasons generates valid sidecars.
- `--dry-run` reports intended output without writing files.
- A second run does not duplicate output and reports skipped/refused existing IDs clearly.
- Script failures leave existing state intact.
- Tests or scripted validation cover dry run, normal run, and duplicate/id collision behavior.

Recommended subskill: agent-factory migrate.

## Evidence

Evidence was not specified in the legacy issue record.
