---
created_at: "2026-06-29T17:40:07.733724397+00:00"
id: "atelier-zjnp"
issue_type: "task"
labels:
- "agent-factory"
- "docs"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make validator guidance derive proof from Outcome"
updated_at: "2026-06-29T17:40:07.733724397+00:00"
---

## Description

Validator guidance tells validators to read the Outcome, decide how to prove or disprove it, run representative checks, and record the result. The planner does not predefine validation scenarios except when a specific contract is itself the work.

## Outcome

Validator guidance makes validation independent: the validator reads the mission or issue `Outcome`, chooses checks that can prove or disprove it, runs those checks, and records the result. Planner-authored validation scenarios are not required unless the exact scenario is the product contract.
