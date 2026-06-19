---
created_at: "2026-06-17T18:00:58.979537811+00:00"
id: "atelier-495r"
issue_type: "task"
labels:
- "guidance"
- "workflow"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T01:17:53.353373937+00:00"
status: "done"
title: "Render actionable PR validator failure guidance"
updated_at: "2026-06-18T01:17:53.353373937+00:00"
---

## Description

Update workflow transition and status output so PR validator failures tell
operators what to do next.

## Outcome

- Missing linked PR failures suggest `atelier pr open <id>`.
- Unmerged linked PR failures suggest `atelier pr status <id>`.
- Transition option output includes PR validator state without exposing raw API
  details.

## Evidence

- Focused CLI transcript proves transition option output for missing and
  unmerged PR validator failures.
- Command transcript shows targeted workflow-output tests pass.
