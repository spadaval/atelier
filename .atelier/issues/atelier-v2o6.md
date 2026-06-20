---
created_at: "2026-06-20T16:54:50.562480578+00:00"
id: "atelier-v2o6"
issue_type: "task"
labels:
- "cutting-pass"
- "mission-collapse"
- "removal"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove mission command namespace after type-aware replacements land"
updated_at: "2026-06-20T16:54:50.562480578+00:00"
---

## Description

Remove mission-specific commands after type-aware issue/workflow replacements
exist. This is the cleanup slice, not the place to invent replacement behavior.

## Outcome

- Mission commands that have replacement issue/workflow/status/link behavior are
  removed from root help and dispatch.
- No compatibility aliases or hidden fallbacks preserve the old command names.
- Docs, command audit, and tests agree on the final mission command surface.

## Evidence

- `target/debug/atelier mission --help` shows only retained commands, or
  `target/debug/atelier mission --help` fails if the namespace is fully removed.
- Removed mission commands fail as unknown commands.
- Focused integration tests prove replacement commands still cover creation,
  status/detail, closeout, notes, and relationships.
