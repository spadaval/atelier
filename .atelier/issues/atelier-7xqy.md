---
created_at: "2026-06-13T02:36:04.942139272+00:00"
id: "atelier-7xqy"
issue_type: "feature"
labels:
- "cli"
- "ux"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ezvf"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T04:23:53.949302917+00:00"
status: "done"
title: "Make mission output compact by default"
updated_at: "2026-06-13T04:23:53.949302917+00:00"
---

## Description

Reduce overwhelming mission output by making default status failure/gap oriented and moving long pass lists and evidence bodies behind explicit drill-down.

## Outcome

- Default mission status shows state, health, blockers, missing proof, active work, and next action without long repeated pass lines.
- Verbose mode can show full audit details when requested.
- Evidence summaries do not hide the mission shape.

## Evidence

- Snapshot or transcript tests compare compact default output and verbose output.
- Manual review uses a mission fixture with many closed children and records enough context to explain why the default output is or is not scannable.
- `atelier lint`, `atelier export --check`, and relevant CLI tests pass.
