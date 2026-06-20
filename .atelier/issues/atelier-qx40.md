---
created_at: "2026-06-19T22:42:56.483178051+00:00"
id: "atelier-qx40"
issue_type: "task"
labels:
- "migration"
- "workflow-yaml"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ih2n"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T01:58:35.432568047+00:00"
status: "done"
title: "Migrate workflow.yaml and starter policy"
updated_at: "2026-06-20T01:58:35.432568047+00:00"
---

## Description

Update `.atelier/workflow.yaml`, starter policy YAML, and workflow examples. Rename awkward workflows, collapse review/validation categories to high-level rollups, add static descriptions to meaningful transitions, add explicit archive transitions or remove archived terminal states, and ensure spike review behavior is coherent.

## Outcome

- The repository workflow, starter policy, and examples use the target schema and vocabulary accepted by the implemented workflow engine.
- Validation and review workflows are purpose-built, readable, and free of stale action or validator names.

## Evidence

- `atelier workflow check` passes.
- `atelier lint` passes.
- Search finds no stale `effects`, old validator names, or stale review_artifact action names.
