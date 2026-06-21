---
created_at: "2026-06-21T16:37:30.763484361+00:00"
id: "atelier-s43l"
issue_type: "feature"
labels:
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-iv2x"
  - kind: "issue"
    id: "atelier-m2ql"
  - kind: "issue"
    id: "atelier-nbhp"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Require mission type declaration and workflow coverage"
updated_at: "2026-06-21T16:37:30.763484361+00:00"
---

## Description

Remove the CLI's built-in `mission` issue type escape hatch. Mission-shaped creation must be valid only when repository workflow policy declares the type and covers it exactly once.

## Outcome

- `atelier issue create --issue-type mission` succeeds only when `.atelier/workflow.yaml` declares `mission` or the chosen objective type.
- A missing mission type produces a workflow policy error naming the required policy location.
- The implementation does not add compatibility aliases or fallback creation paths.

## Evidence

- Positive and negative CLI tests cover declared and undeclared mission type behavior.
- `atelier lint` and focused workflow tests pass.
