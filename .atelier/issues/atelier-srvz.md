---
created_at: "2026-06-14T02:52:49.254925530+00:00"
id: "atelier-srvz"
issue_type: "task"
labels:
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Add active-work repair or reconcile command"
updated_at: "2026-06-14T08:08:24.432071142+00:00"
---

## Description

Provide a first-class command for clearing or reconciling stale active-work associations and missing worktree paths.

## Outcome

Agents can recover local work association without editing runtime state by hand.

## Evidence

CLI help and focused tests cover the repair command and stale association scenario.
