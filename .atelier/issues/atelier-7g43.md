---
created_at: "2026-06-17T17:58:58.707210232+00:00"
id: "atelier-7g43"
issue_type: "epic"
labels:
- "fields"
- "implementation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-495r"
  - kind: "issue"
    id: "atelier-c5oz"
  - kind: "issue"
    id: "atelier-e7oj"
  - kind: "issue"
    id: "atelier-hw9t"
  - kind: "issue"
    id: "atelier-jhzk"
  - kind: "issue"
    id: "atelier-mpah"
  - kind: "issue"
    id: "atelier-onie"
  - kind: "issue"
    id: "atelier-udny"
  - kind: "issue"
    id: "atelier-vg25"
  - kind: "issue"
    id: "atelier-yrwm"
  children:
  - kind: "issue"
    id: "atelier-nmkm"
  - kind: "issue"
    id: "atelier-rgmg"
  - kind: "issue"
    id: "atelier-x1fn"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Add typed issue fields"
updated_at: "2026-06-17T18:00:03.988795750+00:00"
---

## Description

Add the typed issue-field foundation needed for `forge_pr` and future
repository-defined work-item metadata.

## Outcome

- `.atelier/workflow.yaml` schema version 2 supports strict typed field
  definitions.
- Issue canonical Markdown can store validated field values.
- The `forge_pr` field is available for one active Forgejo PR on an epic or
  standalone issue.

## Evidence

- Focused tests cover workflow field parsing and issue field
  parse/render/rebuild behavior.
- Command transcript shows targeted tests plus `atelier lint` pass for typed
  field records.
