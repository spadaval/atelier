---
created_at: "2026-06-13T20:37:01.941584397+00:00"
id: "atelier-rgd1"
issue_type: "task"
labels:
- "cli"
- "ux"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Audit focused output for common operator workflows"
updated_at: "2026-06-13T20:37:01.941584397+00:00"
---

## Description

Review default output for status, mission status/show/list, issue show/list, evidence record/show/list, dependency/link/graph, worktree, lint, doctor, export, and rebuild. Identify output that exposes too much diagnostic detail, too little next-action context, or command lists that are not tied to the current purpose.

## Outcome

- Each audited command has a concise default answer and a documented path for more detail.
- Next actions are intent-labeled and do not send ordinary operators into low-level repair commands unless health is actually degraded.
- Quiet output remains minimal and composition-friendly.

## Evidence

- Command transcript artifact captures representative healthy, empty, blocked, degraded, and closeout-ready states.
- Tracker issue links or review artifact list every failed output classification and its follow-up implementation issue.
- Focused output tests or snapshot tests cover changed behavior.
