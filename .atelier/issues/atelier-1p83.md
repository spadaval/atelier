---
created_at: "2026-06-12T05:12:21.600979854+00:00"
id: "atelier-1p83"
issue_type: "task"
labels:
- "closeout"
- "reliability"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9pkx"
  - kind: "issue"
    id: "atelier-pyre"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Require independent epic and mission closeout audits"
updated_at: "2026-06-12T23:22:02.887038668+00:00"
---

## Description

Require independent closeout audits for epics and missions. Closed children are
necessary but not sufficient proof that the parent outcome is real.

## Outcome

- Epic closeout checks all children closed, parent Outcome satisfied, evidence
  attached, and no parent/child contradictions.
- Mission closeout checks all linked work plus mission-level outcomes and
  evidence.
- Closeout output names the missing proof rather than emitting generic success.
- Agents cannot close an epic or mission by only closing child issues.

## Evidence

- Tests prove closed children alone do not close an epic or mission when the
  parent Outcome or mission validation criteria lack attached proof.

- Tests prove missing parent evidence, unresolved child contradictions, and open
  linked work block closeout with specific diagnostics.

- Run focused closeout tests for epic and mission targets.
