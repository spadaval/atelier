---
created_at: "2026-06-19T20:14:22.363363767+00:00"
id: "atelier-yrql"
issue_type: "feature"
labels:
- "planning"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-s8n2"
  - kind: "issue"
    id: "atelier-xiqw"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Add transition effect planning and preflight"
updated_at: "2026-06-19T20:59:42.069285905+00:00"
---

## Description

Add a transition planning/preflight layer that turns a requested workflow
transition into validators, required fields, planned effects, preflight checks,
and recovery guidance without mutating records.

## Outcome

- The planner exposes a deterministic transition plan for CLI rendering and
  execution code.
- Planned effects include stable names, ordered execution, target issue or epic,
  review artifact target when relevant, confirmation requirements, and skip or
  block reasons.
- Existing transitions without effects still plan and render as before except
  for any intentionally improved output shape.

## Evidence

- Focused tests cover allowed, blocked, and invalid transition plans with and
  without effects.
- Source review or test names show planning is side-effect free.
- `atelier lint atelier-yrql` passes.
