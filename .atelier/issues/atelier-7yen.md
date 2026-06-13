---
created_at: "2026-06-12T05:12:10.905602262+00:00"
id: "atelier-7yen"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "evidence"
- "reliability"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-diom"
  - kind: "issue"
    id: "atelier-pvuz"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-7hec"
    role: "validates"
  - kind: "evidence"
    id: "atelier-cyo8"
    role: "validates"
  - kind: "evidence"
    id: "atelier-fqzu"
    role: "validates"
  - kind: "evidence"
    id: "atelier-m7ly"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T21:19:32.235226646+00:00"
status: "done"
title: "Capture and attach validation evidence from commands"
updated_at: "2026-06-12T21:19:32.235226646+00:00"
---

## Description

Make evidence collection cheap and explicit so agents attach real proof instead
of writing summaries that cannot be audited.
- Operators can capture command output as an evidence record without manual
  copy/paste.
- Evidence records can be attached to issues, epics, and missions in the same
  workflow.
- Evidence summaries identify command, exit status, timestamp, and target.
- Failed commands can be captured as failing or blocked evidence without being
  mistaken for pass proof.
- CLI tests cover capturing passing, failing, and blocked command transcripts as
  evidence records and attaching them to issues, epics, and missions.

- Evidence show/list output exposes enough metadata for closeout audits.

- Tests prove captured evidence records include command, exit status, timestamp,
  target, result, and bounded output summary.

- Run focused evidence command tests.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
