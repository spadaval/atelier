---
created_at: "2026-06-19T18:25:45.542222807+00:00"
id: "atelier-zr6j"
issue_type: "epic"
labels:
- "schema"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-298c"
  - kind: "issue"
    id: "atelier-qsmn"
  - kind: "issue"
    id: "atelier-um8u"
  children:
  - kind: "issue"
    id: "atelier-68sm"
  - kind: "issue"
    id: "atelier-yrql"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Workflow schema and transition planner"
updated_at: "2026-06-19T21:02:48.229908271+00:00"
---

## Description

Extend workflow policy parsing and transition planning so transition effects are
declared, validated, and previewed before any command executes them. This epic
owns the static contract and planning model, not effect execution.

## Outcome

- Workflow schema supports a constrained transition effect declaration with
  stable names, parameters, ordering, and strict validation errors.
- Invalid effect names, invalid parameters, unsupported workflow/review-mode
  combinations, and ambiguous effect placement are rejected before runtime.
- A transition planner returns validators, required fields, planned effects,
  confirmation or preflight requirements, and recovery text in a deterministic
  structure reusable by CLI rendering and the execution engine.
- Parser and planner changes do not execute effects or mutate issue records.

## Evidence

- Focused tests cover valid and invalid effect declarations, transition plan
  output, and review-mode-specific effect validation.
- Schema or product documentation examples include the v1 review effect without
  broadening review commands into workflow transition commands.
- `atelier lint atelier-zr6j` passes after child work lands.
