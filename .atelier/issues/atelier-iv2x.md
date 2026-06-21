---
created_at: "2026-06-21T16:37:30.769361494+00:00"
id: "atelier-iv2x"
issue_type: "feature"
labels:
- "migration"
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kka3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Migrate existing first-class mission records to target objective records"
updated_at: "2026-06-21T19:23:02.679633459+00:00"
---

## Description

Implement the committed-state migration for existing `.atelier/missions/*.md` records after the target mission/objective storage shape is settled.

## Outcome

- Mission IDs, titles, intent, constraints, risks, validation, notes, terminal notes, labels, and relationships survive migration.
- Mission work and blocker links keep their precise `advances` and `blocked_by` meanings.
- The migration does not keep a compatibility reader for old mission paths after the direct migration is complete.

## Evidence

- Fixture tests compare pre/post migration canonical records and projection rows.
- Repository migration transcript names created/changed files and `atelier lint` passes.
