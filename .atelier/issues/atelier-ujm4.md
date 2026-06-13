---
created_at: "2026-06-10T16:00:59.269185814+00:00"
id: "atelier-ujm4"
issue_type: "task"
labels:
- "activity"
- "evidence"
- "links"
- "record-store"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1z0u"
  - kind: "issue"
    id: "atelier-krhk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T17:35:56.728302758+00:00"
status: "done"
title: "Record evidence attachments in issue activity"
updated_at: "2026-06-10T17:35:56.728302758+00:00"
---

## Description

Bridge rich evidence records into issue activity history without flattening evidence.

What:
- Keep rich evidence records and artifact pointers in `.atelier-state/evidence/`.
- When evidence is attached or made relevant to an issue, add an issue activity entry with `event_type: evidence_attached`.
- Include lightweight reference metadata such as `evidence_id` and `result` in the activity front matter.
- Ensure history rendering can show the reference inline and identify the corresponding `atelier evidence show` target.

Out of scope:
- Redesigning evidence storage or artifact backends.
- Moving evidence bodies into issue activity files.

## Outcome

- Attaching evidence to an issue creates a valid `evidence_attached` activity entry.
- The entry includes enough metadata for human-readable history and JSON consumers to locate the rich evidence record.
- Tests cover pass/fail or result-bearing evidence references and missing evidence diagnostics.

Recommended subskill: agent-factory implement.

## Evidence

Evidence was not specified in the legacy issue record.
