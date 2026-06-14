---
created_at: "2026-06-13T17:29:11.074212380+00:00"
id: "atelier-3z35"
issue_type: "feature"
labels:
- "workflow"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9t3z"
  - kind: "issue"
    id: "atelier-ewpk"
  - kind: "issue"
    id: "atelier-fyms"
  - kind: "issue"
    id: "atelier-jwcz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T19:21:14.603079799+00:00"
status: "done"
title: "Replace start close and abandon workflow commands"
updated_at: "2026-06-13T19:21:14.603079799+00:00"
---

## Description

Rework the ergonomic command wrappers around configured workflow transitions. Start should become both a workflow transition and local active-work association; close should become a terminal workflow transition; finish should be replaced by explicit abandon semantics.
- atelier start <id> runs the configured start transition into an in-progress-category status and records local active work.
- atelier issue close <id> --to <status> --reason <text> runs a configured terminal done-category transition, requiring --to only when the done target is ambiguous.
- atelier abandon [id] --reason <text> clears local active work without changing issue workflow status and records why in issue activity.
- Root and hidden work finish surfaces are removed or replaced without compatibility aliases unless a later human request explicitly asks for a window.
- CLI tests cover start success, start validator failure, close success, ambiguous done target rejection, archive behavior, and abandon reason requirements.
- Help/status/prime transcript no longer describes finish as the normal success path.
- atelier lint, atelier export --check, and focused workflow command tests pass.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
