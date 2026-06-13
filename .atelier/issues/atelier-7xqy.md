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
status: "open"
title: "Make mission output compact by default"
updated_at: "2026-06-13T02:36:04.942139272+00:00"
---

## Description

Reduce overwhelming mission output by making default status failure/gap oriented and moving long pass lists and evidence bodies behind explicit drill-down.

## Outcome

- Default mission status shows state, health, blockers, missing proof, active work, and next action without long repeated pass lines.
- Verbose mode can show full audit details when requested.
- Evidence summaries do not hide the mission shape.

## Evidence

- Snapshot or transcript tests compare compact default output and verbose output.
- Manual review uses a mission fixture with many closed children, records the reviewer role, baseline output, decision criteria, and pass/fail rationale for scannability.
- `atelier lint`, `atelier export --check`, and relevant CLI tests pass.
