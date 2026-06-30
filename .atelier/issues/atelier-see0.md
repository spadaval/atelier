---
created_at: "2026-06-29T17:39:13.926907534+00:00"
id: "atelier-see0"
issue_type: "task"
labels:
- "cli"
- "mission"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Derive mission status vocabulary from workflow policy"
updated_at: "2026-06-29T17:39:13.926907534+00:00"
---

## Description

Mission commands stop hardcoding valid mission statuses, current/terminal classification, and status ordering. They derive those answers from .atelier/workflow.yaml status categories, initial status, done statuses, and configured transitions.

## Outcome

Mission commands derive valid statuses, terminal/current classification, and lifecycle ordering from `.atelier/workflow.yaml`. Removing or renaming a mission status in workflow policy does not leave stale mission-specific Rust allowlists behind.
