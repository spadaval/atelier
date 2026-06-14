---
created_at: "2026-06-14T02:52:36.969281137+00:00"
id: "atelier-oqtz"
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
status: "todo"
title: "Improve stale projection and invalid canonical recovery messages"
updated_at: "2026-06-14T02:52:36.969281137+00:00"
---

## Description

Make stale projection and invalid canonical Markdown failures report one ordered recovery path and preserve the original blocking file or command context.

## Outcome

Agents can recover from projection freshness failures without cycling through export, rebuild, lint, and read commands blindly.

## Evidence

Focused tests or validation transcripts cover stale projection and invalid canonical record messages.
