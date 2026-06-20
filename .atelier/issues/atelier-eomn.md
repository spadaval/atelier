---
created_at: "2026-06-20T04:17:09.566876467+00:00"
id: "atelier-eomn"
issue_type: "task"
labels:
- "status"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-l7i6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make status guidance derive from transition options"
updated_at: "2026-06-20T04:17:09.566876467+00:00"
---

## Description

Imported bundle issue.

## Outcome

- `atelier status` stops inventing static next lifecycle commands.
- Current-work guidance is derived from transition options, blockers, validators, and action preflight state.
- Dashboard output remains concise while avoiding impossible or stale next steps.

## Evidence

- `atelier status` output for active/current work is backed by the transition option engine or routes to it without static lifecycle prompts; focused tests cover invalid start/review-request suggestions.
