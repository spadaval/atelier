---
created_at: "2026-06-13T17:29:11.074541262+00:00"
id: "atelier-fyms"
issue_type: "validation"
labels:
- "assignee:root"
- "validation"
- "workflow"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2bpd"
  - kind: "issue"
    id: "atelier-q5r6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Independently validate repo-defined workflow behavior"
updated_at: "2026-06-13T19:52:45.616632362+00:00"
---

## Description

Independently validate the completed workflow mission behavior without fixing implementation defects in this item. The validator should exercise the user-visible workflow path and classify each mission behavior as pass, fail, blocked, deferred, or not applicable.

## Outcome

- Starter policy, workflow check, status migration, start transition, blocked transition, close with evidence, lightweight spike close, archive, missing YAML, and unmigrated-record failures are independently evaluated.
- CLI output is understandable for future agents and does not require raw workflow validator commands.
- Any discovered defect is recorded as a follow-up implementation issue instead of being silently fixed during validation.

## Evidence

- First-class validation evidence contains the scenario transcript and line-by-line outcome classification.
- Attached validation evidence record names residual risks or follow-up issue IDs.
- atelier lint, atelier export --check, atelier doctor, and relevant focused tests are included in the validation evidence or explicitly classified.
