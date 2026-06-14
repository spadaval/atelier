---
created_at: "2026-06-14T02:52:21.445933576+00:00"
id: "atelier-jqcb"
issue_type: "task"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Require claim-specific closeout proof mapping"
updated_at: "2026-06-14T07:05:30.354943729+00:00"
---

## Description

After the Agent Factory/Atelier guidance boundary is reconciled, update
portable closeout guidance so workers name the specific Outcome or Evidence
line proved by their evidence, especially for broad or parent-level claims.

## Outcome

Proof summaries are claim-specific rather than broad green-test statements.

## Evidence

- File diff in Agent Factory closeout guidance includes the claim-specific proof requirement and examples.
- File diff or review artifact shows examples routing repository-specific
  closeout diagnostics to Atelier-owned mission status/audit and evidence
  surfaces.
- `git diff --check` passes for the documentation change.
