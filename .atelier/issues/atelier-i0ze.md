---
created_at: "2026-06-29T17:39:33.980064328+00:00"
id: "atelier-i0ze"
issue_type: "task"
labels:
- "mission"
- "tests"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T15:17:25.756909126+00:00"
status: "done"
title: "Add regression tests for workflow-configured mission lifecycle"
updated_at: "2026-06-30T15:17:25.756909126+00:00"
---

## Description

Tests prove mission initial status, allowed transitions, current/terminal filtering, and dashboard ordering follow workflow config. A fixture workflow change should alter mission behavior without changing Rust status constants.

## Outcome

Regression tests prove mission lifecycle behavior follows workflow config for initial status, transitions, current/terminal filtering, and dashboard ordering. The tests fail if Rust reintroduces hardcoded mission status vocabulary.
