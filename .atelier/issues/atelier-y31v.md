---
created_at: "2026-06-17T18:00:12.896603007+00:00"
id: "atelier-y31v"
issue_type: "task"
labels:
- "records"
- "sessions"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Add canonical session records and bounded session activity"
updated_at: "2026-06-17T23:53:28.212628088+00:00"
---

## Description

Add canonical session records under `.atelier/sessions/` with activity sidecars
for lifecycle events and bounded command transcript summaries.

## Outcome

- Session records store agent identity, operator role, optional subskill,
  target issue or mission, status, timestamps, and session kind.
- Session activity stores command line, timestamps, exit status, and bounded
  redacted stdout/stderr summaries.
- Full proof output remains evidence, not session activity.

## Evidence

- Focused RecordStore tests cover session record parse/render/validation and
  bounded activity rendering.
- Command transcript shows targeted session record tests pass.
