---
created_at: "2026-06-29T20:15:20.520150409+00:00"
id: "atelier-g87o"
issue_type: "task"
labels:
- "cli"
- "complexity"
- "maintenance"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove or admin-hide destructive maintenance delete"
updated_at: "2026-06-29T20:15:20.520150409+00:00"
---

## Description

The maintenance delete surface is destructive and has little product value as a normal command. It should not be part of the everyday operator surface.

## Outcome

Destructive maintenance delete is removed, hidden behind explicit admin/recovery affordances, or replaced by a safer documented recovery path. Help text, role guides, and tests agree on the decision, and normal workflow output does not suggest it as a routine action.
