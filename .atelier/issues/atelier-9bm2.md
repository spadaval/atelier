---
created_at: "2026-10-17T01:10:57.985221680+00:00"
id: "atelier-9bm2"
issue_type: "feature"
labels:
- "cli-output"
- "work-views"
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
title: "Show outcome, own blockers and next transition in the work mission header"
updated_at: "2026-10-17T01:10:57.985221680+00:00"
---

## Goal

The `work mission` header repeats the status (`atelier-1w8c [mission] draft - …` then `Status: draft`) but omits:
- the Outcome, which `docs/product/issue-inventory-and-mission-overview.md` says the view shows;
- the mission's own blockers: fixture mission atelier-ba3h is `blocked_by atelier-ubti` and its dashboard shows nothing, though the objective `start` transition requires `blockers.transitive_none_open`;
- what moves the mission itself: a draft mission needs `ready`; fixture atelier-j76w is `todo` with one active item and nothing says it is not started.
`work parent` already prints `Transition readiness: start blocked: … Next: atelier issue show atelier-spvd` (crates/atelier-cli/src/commands/work.rs:416-428, 600-645); missions get nothing.

## Done when

`work mission <id>` prints, with no separate Status line:
```
atelier-j76w [mission] todo - Ship the widget pipeline
Outcome : Widgets ship.
Strategy: docs/strategies/x.md
Blocked by: atelier-ubti ready low Audit legacy widget names
Next    : start (1 item already active) — atelier issue transition atelier-j76w start
```
Outcome is the first line of the Outcome section; Strategy and Blocked by appear only when present. Tests cover a draft, a todo-with-active-work and a blocked mission.

## Context

Success criteria and proof are atelier-stsa.
