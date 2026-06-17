---
created_at: "2026-06-17T18:01:01.368821358+00:00"
id: "atelier-cglp"
issue_type: "validation"
labels:
- "cli"
- "tests"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add focused tests for fields, sessions, PR integration, and linked_pr_merged"
updated_at: "2026-06-17T18:01:01.368821358+00:00"
---

## Description

Add focused tests for the implementation surfaces introduced by this mission.

## Outcome

- Tests cover workflow schema v2 fields and canonical issue field handling.
- Tests cover session auto-start, conflict failure, explicit reuse, and end.
- Tests cover Forgejo PR command behavior with mocked responses.
- Tests cover `linked_pr_merged`.

## Evidence

- Command transcript shows focused CLI/unit tests for fields, sessions, PR
  integration, and `linked_pr_merged` pass.
- Test file diff or review artifact names the focused scenarios added.
