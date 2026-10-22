---
created_at: "2026-10-17T20:10:21.682260002+00:00"
id: "atelier-mg7w"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Offer plan approval in the brief footer only when none is recorded"
updated_at: "2026-10-17T20:10:21.682260002+00:00"
---

## Goal

The brief's Next footer suggests recording a plan approval even after one exists (independent validation of atelier-r6bt, S5).

## Done when

With an approval recorded, `atelier brief` offers a different next step, shown by a CLI test.
