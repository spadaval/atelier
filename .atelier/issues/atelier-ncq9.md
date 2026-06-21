---
created_at: "2026-06-21T16:37:30.762733620+00:00"
id: "atelier-ncq9"
issue_type: "epic"
labels:
- "mission-rework"
review:
  kind: pull_request
  number: 18
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-kivn"
  - kind: "issue"
    id: "atelier-m2ql"
  - kind: "issue"
    id: "atelier-s43l"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Move mission lifecycle into workflow policy"
updated_at: "2026-06-21T19:06:23.804657005+00:00"
---

## Description

Remove hidden mission lifecycle rules and make all mission-like status, readiness, and terminal behavior explicit in repository-owned workflow policy.

## Outcome

- The repository must declare the `mission` type or objective type before mission-shaped records can be created.
- Mission/objective transitions use configured workflow transitions, required fields, validators, and actions rather than mission-specific command logic.
- Unknown mission validators, actions, or issue types fail with actionable workflow diagnostics.

## Evidence

- Focused workflow policy parser and CLI tests prove declared mission type coverage and rejection of undeclared mission behavior.
- `cargo nextest run` focused tests and `atelier lint` pass.
