---
created_at: "2026-07-06T17:20:25.694804491+00:00"
id: "atelier-g5fl"
issue_type: "validation"
labels:
- "cli"
- "independent-validation"
- "inventory"
- "mission-dashboard"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate issue inventory and Mission Overview behavior"
updated_at: "2026-07-06T17:20:25.694804491+00:00"
---

## Description

Independently validate the delivered command split from the mission Outcome and public contracts. Exercise representative mission, epic, child, direct-linked, unassigned, blocked, done, and empty tracker states; compare interactive color with `NO_COLOR` and captured noninteractive output; and inspect help and durable guidance. Treat any prior closed implementation claim as context, not proof of current behavior.

## Outcome

- The validator records pass, fail, blocked, or deferred for each mission Outcome claim and cites the command transcript, test result, diff location, or evidence record used for judgment.
- Validation proves that `issue list` is flat and inventory-shaped; `work missions` hides done missions by default, groups linked epics without default leaf rows, accounts for exceptional work, and preserves meaning with and without color; help, docs, and tracker health agree with the implementation.

## Evidence

Evidence was not specified in the bundle.
