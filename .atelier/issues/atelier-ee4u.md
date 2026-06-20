---
created_at: "2026-06-19T22:42:56.466427189+00:00"
id: "atelier-ee4u"
issue_type: "task"
labels:
- "migration"
- "validators"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v4ah"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Rename validators to namespaced policy names"
updated_at: "2026-06-20T00:53:25.251800887+00:00"
---

## Description

Rename validators to namespaced policy names and migrate the starter workflow, docs, fixtures, and tests. Proposed mappings include `evidence.attached`, `blockers.none_open`, `lint.none_blocking`, `tracker.current`, `git.worktree_clean`, `review.complete`, and `children.proof_complete`.

## Outcome

- Workflow validators use namespaced policy names consistently in schema, starter policy, docs, fixtures, and tests.
- Old flat validator names are rejected rather than kept as aliases.

## Evidence

- Parser tests reject old names and accept new names.
- .atelier/workflow.yaml and starter policy use only namespaced validators.
