---
created_at: "2026-10-17T01:10:57.966699407+00:00"
id: "atelier-oxnw"
issue_type: "feature"
labels:
- "cli-output"
- "start"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Make the refused start report one line per blocker"
updated_at: "2026-10-17T01:10:57.966699407+00:00"
---

## Goal

A refused `atelier start` lists one line per unmet condition, such as `[ ] atelier-7yy9 is done (it is ready)`, but never says what the blocker is, so the agent then runs `show` on it (checked 2026-11-11 with the new binary in a converted scratch clone).

## Done when

- [ ] A refused `start` names each blocker's title beside its ID and status, e.g. `[ ] atelier-7yy9 is done (it is ready): Record which thread ... ran each atelier command`.
- [ ] A CLI test covers a refusal with one blocker.

## Context

atelier-7n5u computes in-flight branches for ready work; reuse it if it has landed.

Rewritten for the new tool in the cutover sort (atelier-5zre): the new `start` already prints one line per unmet condition with no repeated error; `--verbose`, `issue transition` and parent-inherited blockers went with the old commands and parents.
