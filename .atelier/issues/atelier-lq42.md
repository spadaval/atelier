---
created_at: "2026-10-16T18:13:26.664844677+00:00"
id: "atelier-lq42"
issue_type: "bug"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Supersede succeeds on a child of a blocked parent, then land rejects it"
updated_at: "2026-10-16T18:13:26.664844677+00:00"
---

## Goal

`atelier issue transition <id> supersede` accepts a task whose parent is blocked, but `atelier land` then fails with `blockers.transitive_none_open: completion blockers: <parent blocker>`. Found in astromech on 2026-10-16: superseding astromech-4f6x, a child of the parked feature astromech-94j4, which waits on astromech-qz9u. Superseding means the work will not happen, so open blockers inherited from the parent should not hold it. Transition and land must agree in either case.

## Done when

A test supersedes a child of a blocked parent, and `land` then passes. If the decision is that land should keep rejecting it, the transition refuses instead, with the same message.
