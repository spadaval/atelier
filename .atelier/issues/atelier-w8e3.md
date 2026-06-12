---
created_at: "2026-06-12T05:11:58.957324222+00:00"
id: "atelier-w8e3"
issue_type: "task"
labels:
- "closeout"
- "mission"
- "reliability"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1p83"
  - kind: "issue"
    id: "atelier-9pkx"
  - kind: "issue"
    id: "atelier-k9m8"
  - kind: "issue"
    id: "atelier-pyre"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add mission contract audit closeout command"
updated_at: "2026-06-12T23:18:39.342474480+00:00"
---

## Description

Add a mission closeout audit that verifies mission outcomes against linked work
and attached evidence instead of only checking that records are syntactically
fresh.

## Outcome

- A mission closeout/audit command lists every mission Outcome item and its
  current proof status.
- Outcome items without linked work or attached evidence are blocking failures.
- The audit distinguishes open work, missing evidence, stale docs/tests, and
  unresolved blockers.
- Mission closeout uses this audit before allowing status `closed`.

## Evidence

- CLI transcript tests cover missing proof, partial proof, and ready-to-close

mission states.

- Tests prove a mission with unproven outcomes cannot close.

- Run focused mission closeout tests.
