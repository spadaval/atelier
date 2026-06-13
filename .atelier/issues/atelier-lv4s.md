---
created_at: "2026-06-13T17:29:11.073812301+00:00"
id: "atelier-lv4s"
issue_type: "feature"
labels:
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fmri"
  - kind: "issue"
    id: "atelier-fyms"
  - kind: "issue"
    id: "atelier-n0p4"
  - kind: "issue"
    id: "atelier-y041"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T18:06:21.719386675+00:00"
status: "done"
title: "Implement workflow policy parser and checker"
updated_at: "2026-06-13T18:06:21.719386675+00:00"
---

## Description

Implement workflow policy loading and validation for .atelier/workflow.yaml. The checker should validate strict schema shape, status categories, transition references, validator definitions and parameters, guidance templates, issue-type mappings, and every current issue status against the selected workflow.

## Outcome

- atelier workflow check reports policy and record health for .atelier/workflow.yaml.
- Invalid YAML, unknown fields, unknown references, missing categories, invalid issue-type mappings, and issues whose status is not valid for their selected workflow are rejected with actionable messages.
- Workflow policy is parsed per command and is not projected into SQLite in v1.

## Evidence

- Parser and checker unit tests cover valid policy and representative invalid policies.
- CLI transcripts show workflow check passing and failing with stable, actionable errors.
- atelier lint and atelier export --check pass after implementation.
