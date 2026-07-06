---
created_at: "2026-06-25T16:22:46.100150183+00:00"
id: "atelier-lijo"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-5ylh"
  - kind: "issue"
    id: "atelier-74px"
  - kind: "issue"
    id: "atelier-7z8w"
  - kind: "issue"
    id: "atelier-hr1x"
  - kind: "issue"
    id: "atelier-lesf"
  - kind: "issue"
    id: "atelier-x2ad"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jcyc"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "done"
title: "Design and implement work dashboards"
updated_at: "2026-06-25T16:23:24.310924016+00:00"
---

## Description

Define and implement the `work` command as the dashboard namespace for operational multi-issue views. `work` itself lists available dashboards, `work queue` replaces current `issue list` dashboard behavior with mission-aware grouping and filters, `work mission <id>` gives orchestrators live mission control state, `work epic <id>` gives branch/review-boundary execution state, and `issue list` is removed after replacement coverage lands.

## Outcome

Atelier's multi-issue dashboards live under `work`: `work` lists dashboards, `work queue` owns repo-wide operational queue views and filters, `work mission <id>` owns live mission orchestration state, and `work epic <id>` owns epic execution/review-boundary state. `issue list` no longer appears in help, docs, role guidance, or dispatch after equivalent `work` behavior is validated.
