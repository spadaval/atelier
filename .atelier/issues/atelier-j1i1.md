---
created_at: "2026-06-19T03:58:28.228799532+00:00"
id: "atelier-j1i1"
issue_type: "feature"
labels:
- "review"
- "schema"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T04:39:07.781830049+00:00"
status: "done"
title: "Add canonical review room YAML records and projections"
updated_at: "2026-06-19T04:39:07.781830049+00:00"
---

## Description

Introduce canonical review room YAML records under `.atelier/reviews/`. This
issue owns record parsing, deterministic rendering, projection indexing, lint,
and doctor diagnostics for room files.

## Outcome

- `.atelier/reviews/<id>.yaml` supports schema `atelier.review`,
  `schema_version: 1`, room metadata, owner issue/epic, source and target
  branches, opened head, and ordered events.
- Supported events are `opened`, `commented`, `finding_opened`,
  `finding_resolved`, `approved`, `changes_requested`, and `merged`.
- Projection and human views derive current state from metadata plus events,
  rather than storing duplicate snapshots.
- Lint and doctor report malformed room files, duplicate event or finding IDs,
  missing owners, invalid anchors, and room IDs that do not match filenames.

## Evidence

- Fixture tests parse and render valid room records deterministically.
- Negative tests prove malformed room YAML is rejected by rebuild/lint paths
  with corrective messages.
- `atelier lint`, `atelier doctor`, and focused room-record tests pass.
