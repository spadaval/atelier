---
created_at: "2026-06-20T15:10:39.150670129+00:00"
id: "atelier-ikuv"
issue_type: "epic"
labels:
- "superseded-planning"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Superseded umbrella: Command surface consolidation and removal"
updated_at: "2026-06-20T16:50:30.672691136+00:00"
---

## Description

Track the command-surface cutting pass from the refreshed command audit. This epic covers consolidation targets that reduce visible commands, delete retired implementation paths, and move shared product contracts out of CLI-specific code.

## Outcome

Atelier has fewer visible and internal command surfaces, with duplicated behavior folded into stronger issue, mission, review, and status views. Retired commands have no dispatch path, no compatibility aliases, and no active tests preserving them.

## Evidence

- `target/debug/atelier --help` shows only supported root commands.
- `target/debug/atelier workflow check` or equivalent drift tests pass after command-audit updates.
- Focused integration tests prove removed commands fail as unknown commands and replacement workflows still work.
