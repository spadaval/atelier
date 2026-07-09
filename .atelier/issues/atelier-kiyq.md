---
created_at: "2026-07-06T17:20:25.690578987+00:00"
id: "atelier-kiyq"
issue_type: "task"
labels:
- "cli"
- "help"
- "inventory"
- "tests"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Align issue-list filters, help, and regression coverage"
updated_at: "2026-07-06T17:20:25.690578987+00:00"
---

## Description

Align the `issue list` parser, help, metadata filters, quiet output, next-action guidance, and regression tests with the simple inventory contract. Remove operational ready/blocked forms from this command when the approved contract routes those questions to `work`, without compatibility aliases.

## Outcome

- `atelier issue list --help` teaches record inventory and its supported metadata filters, while operational selection is discoverable through the appropriate `work` commands.
- Focused tests cover default inventory, each surviving filter, quiet IDs, empty results, stable ordering, rejected removed forms, and absence of hierarchy or dashboard formatting.

## Evidence

Evidence was not specified in the bundle.
