---
created_at: "2026-06-14T02:52:27.481147296+00:00"
id: "atelier-yyx4"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add type-aware wrong-kind ID errors"
updated_at: "2026-06-14T02:52:27.481147296+00:00"
---

## Description

Improve read and mutation command errors so passing a mission ID to an issue command, or another wrong record kind, reports the actual record kind and the likely correct command.

## Outcome

Wrong-kind ID mistakes become corrective one-step errors instead of generic not-found loops.

## Evidence

Focused CLI tests cover mission ID to issue command and another representative wrong-kind lookup.
