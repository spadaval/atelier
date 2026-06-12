---
created_at: "2026-06-12T05:12:10.905602262+00:00"
id: "atelier-7yen"
issue_type: "task"
labels:
- "cli"
- "evidence"
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Capture and attach validation evidence from commands"
updated_at: "2026-06-12T05:12:10.905602262+00:00"
---

## Description

Make evidence collection cheap and explicit so agents attach real proof instead
of writing summaries that cannot be audited.

## Outcome

- Operators can capture command output as an evidence record without manual
  copy/paste.
- Evidence records can be attached to issues, epics, and missions in the same
  workflow.
- Evidence summaries identify command, exit status, timestamp, and target.
- Failed commands can be captured as failing or blocked evidence without being
  mistaken for pass proof.

## Evidence

- CLI tests cover capturing passing, failing, and blocked command transcripts as
  evidence records and attaching them to issues, epics, and missions.

- Evidence show/list output exposes enough metadata for closeout audits.

- Tests prove captured evidence records include command, exit status, timestamp,
  target, result, and bounded output summary.

- Run focused evidence command tests.
