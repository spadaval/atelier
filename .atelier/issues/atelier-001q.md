---
created_at: "2026-06-09T19:47:13.601126452+00:00"
id: "atelier-001q"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "performance"
- "task"
- "telemetry"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-001r"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T13:11:00.683840448+00:00"
status: "done"
title: "Instrument CLI commands with structured duration events"
updated_at: "2026-06-11T13:11:00.683840448+00:00"
---

## Description

Add a shared command telemetry wrapper that records start, finish, duration, status, command identity, workspace identity, and optional phase timings for every agent-facing command.

## Outcome

successful and failed commands emit records when enabled, respect disabled telemetry, avoid sensitive argument capture by default, and have focused tests.

## Evidence

Evidence was not specified in the legacy issue record.
