---
created_at: "2026-06-21T16:37:30.764223761+00:00"
id: "atelier-m2ql"
issue_type: "feature"
labels:
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2kfb"
  - kind: "issue"
    id: "atelier-kivn"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Replace hardcoded mission terminal checks with configured validators"
updated_at: "2026-06-21T16:37:30.764223761+00:00"
---

## Description

Move mission readiness and terminal checks out of mission-specific close/status code and into reusable workflow validators that operate on declared objective relationships.

## Outcome

- Configured validators can require advancing work, all advancing work terminal, no direct objective blockers, parseable linked work, attached proof where configured, and clean tracker state.
- Validator output names the failing relationship or work item and the next inspection command.
- Mission/objective status reads validator results from the same workflow evaluation path used by transitions.

## Evidence

- Focused tests prove each validator pass/fail path and transition blocking behavior.
- CLI transcript shows `atelier issue transition <mission-id> --options` and `atelier issue status <mission-id>` agree on terminal blockers.
