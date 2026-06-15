---
created_at: "2026-06-14T21:44:50.565785538+00:00"
id: "atelier-l543"
issue_type: "task"
labels:
- "implementation"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3vzm"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T22:22:37.312568569+00:00"
status: "done"
title: "Split issue and epic workflow policies"
updated_at: "2026-06-14T22:22:37.312568569+00:00"
---

## Description

Change the default workflow policy so ordinary implementation issues close with local proof and epics carry review and validation states. Outcome: issue types that are implementation slices no longer require request_review/request_validation before done; epic and closeout types retain review/validation gates. Evidence: workflow policy docs, fixture diff, and focused tests proving transition options and rejected closes.

## Outcome

- Default issue workflow lets ordinary implementation issues close with local proof instead of mandatory review and validation states.
- Epic workflow retains review and validation states before done.
- Workflow policy docs and fixtures distinguish ordinary issues, epics, validation issues, and closeout work.

## Evidence

- `git diff` transcript for workflow policy docs and fixtures shows separated issue and epic workflows.
- Focused CLI test or transcript shows ordinary issue transition options no longer require review before done.
- Focused CLI test or transcript shows epic transition options still require review and validation before done.
