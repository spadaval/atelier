---
acceptance: []
created_at: "2026-06-09T19:47:13.611406647+00:00"
evidence_required: []
id: "atelier-001r"
issue_type: "task"
labels:
- "assignee:root"
- "diagnostics"
- "json"
- "task"
- "telemetry"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add slow-command diagnostics query surface"
updated_at: "2026-06-11T13:14:37.100757410+00:00"
---

Add a command such as `atelier diagnostics slow` that summarizes global telemetry by command, workspace, time window, and duration threshold with stable JSON output.

Acceptance: query output is deterministic for fixtures, supports thresholds/time windows, documents examples, and works when no telemetry exists.
