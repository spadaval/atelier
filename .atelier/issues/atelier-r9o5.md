---
created_at: "2026-06-13T00:45:40.556687481+00:00"
id: "atelier-r9o5"
issue_type: "task"
labels:
- "tests"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair full lifecycle closeout fixture"
updated_at: "2026-06-13T00:45:40.556687481+00:00"
---

## Description

Repair the broad-suite failure in test_full_issue_lifecycle. The lifecycle fixture still expects a bare close command to succeed, but current closeout rules require attached issue proof before an issue can close.

## Outcome

- The full lifecycle integration test supplies valid closeout proof before closing the issue.
- The test still verifies the lifecycle surfaces for create, list, search, close, and show.
- The default broad-suite fail-fast probe advances past the previous full lifecycle failure.

## Evidence

- cargo nextest run test_full_issue_lifecycle passes.
- cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet advances past the previous full lifecycle failure.
- atelier lint, atelier export --check, and git diff --check pass.
