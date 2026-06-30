---
created_at: "2026-06-29T20:10:26.913475599+00:00"
id: "atelier-brqs"
issue_type: "task"
labels:
- "cli"
- "output"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ubf2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Simplify issue transition default output to failed requirements"
updated_at: "2026-06-29T20:10:26.913475599+00:00"
---

## Description

`atelier issue transition <id>` currently prints validator passes, validator messages, action preflights, descriptions, dirty-path dumps, blockers, and commands with similar weight. The default view should stay close to the workflow facts without becoming a debug log.

## Outcome

Default transition output shows each available transition with allowed or blocked state and lists only failed requirements for blocked transitions. Passing validators, long validator messages, action/preflight internals, transition descriptions, dirty path lists, and full recovery diagnostics move behind an explicit verbose/detail view. The renderer does not invent conversational causes; it presents configured transition names and failed requirement names from existing workflow results.
