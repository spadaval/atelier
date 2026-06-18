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
title: "Add focused tests for fields, issue-event attempts, PR integration, and linked_pr_merged"
updated_at: "2026-06-18T16:49:09.148897844+00:00"
---

## Description

Add focused tests for the implementation surfaces introduced by this mission,
updated for the decision that sessions are derived from issue events rather
than durable workflow records.

## Outcome

- Tests cover workflow schema v2 fields and canonical issue field handling.
- Tests cover issue-event attempt metadata, deterministic worker/reviewer/validator
  derivation, and inspection-only `session list/show` projections.
- Tests cover Forgejo PR command behavior with mocked responses.
- Tests cover `linked_pr_merged`.

## Evidence

- Command transcript shows focused CLI/unit tests for fields, issue-event
  attempts, PR integration, and `linked_pr_merged` pass.
- Test file diff or review artifact names the focused scenarios added.
