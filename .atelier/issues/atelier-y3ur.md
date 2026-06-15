---
created_at: "2026-06-15T05:13:36.523358965+00:00"
id: "atelier-y3ur"
issue_type: "task"
labels:
- "markdown"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-rxgn"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T18:09:36.760697164+00:00"
status: "done"
title: "Move RecordStore mutations and relationship rendering behind records API"
updated_at: "2026-06-15T18:09:36.760697164+00:00"
---

## Description

Move RecordStore mutation behavior and relationship rendering behind a clear `atelier-records` API consumed by application and storage code.

## Outcome

- Canonical issue, mission, plan, evidence, and relationship mutations write through `atelier-records`.
- Application and command layers do not duplicate record-kind lists or relationship constructors.
- Atomic write and deterministic render behavior remain intact.

## Evidence

- Mutation tests prove record writes update canonical Markdown first.
- Search transcript shows duplicated record-kind lists or relationship constructors were removed or explicitly classified.
- Representative issue, mission, plan, and evidence mutation CLI tests pass.
