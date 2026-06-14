---
created_at: "2026-06-14T02:52:15.406896648+00:00"
id: "atelier-tje5"
issue_type: "task"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add Agent Factory safe shell recipes"
updated_at: "2026-06-14T02:52:15.406896648+00:00"
---

## Description

After the Agent Factory/Atelier guidance boundary is reconciled, add portable
shell hygiene guidance for cargo test filters, nextest expressions, rg
patterns, command substitution hazards, and python3 usage.

## Outcome

Worker assignments have safe validation recipes for common checks.

## Evidence

- Guidance includes concrete command forms and anti-examples from the transcript
  findings.
- Repository-specific validation command recipes are linked or routed to
  Atelier-owned quality docs instead of duplicated into the portable skill when
  they depend on this repository's current command surface.
