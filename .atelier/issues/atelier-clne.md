---
created_at: "2026-07-28T16:39:31.735055742+00:00"
id: "atelier-clne"
issue_type: "epic"
labels:
- "cli"
- "mission-control"
- "planning"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "draft"
title: "Make strategy and execution horizons legible"
updated_at: "2026-08-05T16:43:27.127727601+00:00"
---

## Description

Make the mission's governing outcome map and its current executable horizon understandable from normal Atelier surfaces without introducing a second plan tracker.

Known constraints: strategy remains ordinary repository Markdown; mission scope remains typed `advances` links plus descendants; normal operators should not need raw file inspection.

Current unknowns: which smallest combination of body conventions, parsing, and presentation is sufficient.

Expansion condition: expand after the adaptive contract is validated and the audit identifies the smallest missing operator behaviors.

## Outcome

- Normal mission inspection surfaces the governing strategy reference and revision when present.
- Operators can distinguish outcome-oriented epics from current ready issues and see an epic's expansion condition without treating every outcome as an executable task.
- Missing optional planning metadata degrades gracefully to ordinary Markdown rather than blocking work.
