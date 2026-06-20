---
created_at: "2026-06-19T22:42:56.471248474+00:00"
id: "atelier-cin6"
issue_type: "epic"
labels:
- "actions"
- "workflow-policy"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kmmv"
  - kind: "issue"
    id: "atelier-qx40"
  children:
  - kind: "issue"
    id: "atelier-0d5k"
  - kind: "issue"
    id: "atelier-cko9"
  - kind: "issue"
    id: "atelier-pv77"
  - kind: "issue"
    id: "atelier-z7vb"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T00:52:25.262842921+00:00"
status: "done"
title: "Epic: Replace effects with transition actions"
updated_at: "2026-06-20T00:52:25.262842921+00:00"
---

## Description

Replace transition `effects` with built-in transition `actions` and move branch/review transition-time behavior into declared actions.

## Outcome

- No separate hooks system exists.
- No hidden branch or review mutation is required to understand a transition.

## Evidence

- Workflow parser accepts `actions` and rejects `effects`.
- Transition option output renders Planned Actions.
- Branch and review mutations used during transitions are declared in workflow actions.
