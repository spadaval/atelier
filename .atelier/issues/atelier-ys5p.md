---
created_at: "2026-06-12T19:29:21.552279861+00:00"
id: "atelier-ys5p"
issue_type: "task"
labels:
- "migration"
- "mission"
- "records"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-6aor"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Migrate mission records away from escaped data JSON"
updated_at: "2026-06-12T19:29:21.552279861+00:00"
---

## Description

Convert existing mission records from escaped front matter JSON to the readable
mission record contract. The repository should stop carrying mission state in a
format that is durable only for machines.

## Outcome

- Existing `.atelier/missions/*.md` records are migrated to the new shape.
- Rebuild and export/check use the migrated records as canonical state.
- Legacy escaped mission `data` records are migrated directly or rejected with
  concrete recovery guidance.
- No compatibility writer continues emitting escaped mission `data` JSON.

## Evidence

- Migration transcript and representative migrated mission diff.
- Fixture or integration test covering an old escaped-JSON mission record.
- `atelier lint`, `atelier export --check`, and `atelier doctor` output after a
  clean rebuild from migrated records.
