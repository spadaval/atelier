---
created_at: "2026-06-20T16:54:46.612684053+00:00"
id: "atelier-djoq"
issue_type: "task"
labels:
- "cutting-pass"
- "mission-collapse"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v2o6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Move mission close and focus behavior into workflow decisions"
updated_at: "2026-06-20T16:54:46.612684053+00:00"
---

## Description

Move mission closeout and active-focus behavior into explicit workflow
decisions. Mission completion should use the workflow validator/action system;
mission focus/start should be removed unless a concrete durable lifecycle use
survives design review.

## Outcome

- Mission closeout is modeled as a transition on the mission objective type
  with validators/actions instead of `mission close` bespoke code.
- Transition note/reason input records close rationale for the mission
  transition.
- Active mission focus/start is removed or replaced with a non-lifecycle
  mechanism that does not masquerade as mission status.

## Evidence

- `.atelier/workflow.yaml` and `docs/product/workflow-configuration.md` show
  the mission objective close transition and gates.
- Focused tests prove mission close replacement blocks on open work, blockers,
  missing proof, and dirty checkout when configured.
- `mission start` or its replacement decision is documented in the command
  audit with observable removed/retained behavior.
