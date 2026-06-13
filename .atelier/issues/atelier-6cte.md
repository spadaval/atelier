---
created_at: "2026-06-13T02:36:01.510687439+00:00"
id: "atelier-6cte"
issue_type: "task"
labels:
- "cli"
- "docs"
- "mission"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-7xqy"
  - kind: "issue"
    id: "atelier-b7wl"
  - kind: "issue"
    id: "atelier-yj4c"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Define mission operator command contract"
updated_at: "2026-06-13T02:36:01.510687439+00:00"
---

## Description

Define the reduced mission/operator command model before implementation. Decide whether mission audit becomes a status mode, a closeout mode, or an advanced diagnostic, and document the normal operator path.

## Outcome

- The command contract identifies the primary mission status surface and its default, verbose, and closeout/audit behaviors.
- Normal output requirements are defined for draft, ready, active, blocked, close-ready, and closed missions.
- Help and docs describe when to use drill-down commands instead of raw workflow diagnostics.

## Evidence

- File-change review of product or architecture docs shows the command contract.
- Transcript artifacts or expected-output test fixtures cover each mission state.
- Review artifact records the human choice on the fate of `mission audit` before dependent implementation begins.
