---
created_at: "2026-06-12T19:29:10.963980302+00:00"
id: "atelier-8ec6"
issue_type: "task"
labels:
- "assignee:root"
- "implementation"
- "mission"
- "records"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ys5p"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-53nl"
    role: "validates"
  - kind: "evidence"
    id: "atelier-fg6q"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Introduce typed mission record parsing and rendering"
updated_at: "2026-06-12T22:01:26.799111981+00:00"
---

## Description

Implement mission-specific parsing and rendering for the readable record
contract. Mission commands should work against a typed mission model instead of
requiring each caller to know the shape of a generic JSON blob.

## Outcome

- Mission create, update, show, status, rebuild, export/check, lint, and
  projection refresh read and write the new mission record contract.
- Mission semantics are represented by typed structures or equivalent
  mission-specific APIs, not raw `serde_json::Value` plumbing in command code.
- New mission records round-trip deterministically.
- The obsolete escaped mission `data` writer is removed.

## Evidence

- Parser and renderer tests for the new mission record shape.
- Round-trip tests proving deterministic output.
- Focused command tests or transcripts for mission create, update, show, and
  status using the new model.
