---
created_at: "2026-06-29T17:53:55.606250281+00:00"
id: "atelier-siu5"
issue_type: "task"
labels:
- "agent-factory"
- "docs"
- "planning"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Define record-shape and scope guidelines for planners"
updated_at: "2026-06-29T17:53:55.606250281+00:00"
---

## Description

Planner-facing guidance clearly explains when to use a mission, epic, ordinary issue, validation issue, or evidence record, and what each record should contain. The Agent Factory planning procedure owns the planning instructions; `atelier man manager` and role surfaces own the command-facing summary; product docs stay as backing reference.

## Outcome

Planner and manager guidance gives agents a short, concrete record-shape guide:
missions define target state and scope, where mission scope is direct
`advances` links to root work plus the descendants of those roots; epics group
coherent reviewable work; ordinary issues own one accountable change;
validation issues own independent judgment; and evidence records are receipts
from checks that actually ran. The guidance lives primarily in Agent Factory
planning instructions and `atelier man` role surfaces, with product docs kept
as backing reference rather than the first place agents must read.
