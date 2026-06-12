---
created_at: "2026-06-12T20:29:35.180566310+00:00"
id: "atelier-cany"
issue_type: "task"
labels:
- "cli"
- "implementation"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-a4sn"
  - kind: "issue"
    id: "atelier-trr2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Implement issue transition options surface"
updated_at: "2026-06-12T20:29:35.180566310+00:00"
---

## Description

Implement the user-facing issue transition options surface specified in
`atelier-vr9g`. Operators should be able to ask what an issue can do next
without knowing internal workflow validator names.

## Outcome

- A command such as `atelier issue transition <id> --options` or the approved
  equivalent lists allowed actions, blocked actions, fast gate reasons, and the
  command to perform each allowed action.
- Issue show includes compact transition readiness without dumping validator
  internals.
- Missing evidence, open blockers, malformed sections, active work, and closed
  issue states are shown as actionable transition blockers.
- Expensive proof remains in evidence records and validation transcripts, not
  synchronous transition calculation.
- Help text positions raw workflow validation as advanced/internal rather than
  normal next-action discovery.

## Evidence

- CLI transcript tests for open ready issue, blocked issue, closed issue,
  active-work issue, close-ready issue, and missing-evidence close block.
- Consistency check between issue show, transition/options output, and closeout
  behavior.
- Help transcript proving the transition surface is discoverable.
