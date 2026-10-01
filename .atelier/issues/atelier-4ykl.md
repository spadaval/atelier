---
created_at: "2026-10-01T21:19:17.729952891+00:00"
id: "atelier-4ykl"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-xz11"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-01T22:45:47.903529049+00:00"
status: "done"
title: "Align product and architecture documentation"
updated_at: "2026-10-01T22:45:47.903529049+00:00"
---

## Goal

Make product and architecture guidance describe the implemented repo-native records, commands and invariant guards.

## Done when

Independent docs review confirms current command examples, typed schema v2, review/evidence semantics and bundle fixtures agree with the integrated CLI.

## Scope

PRODUCT_INTENT.md, CONTEXT.md, docs and the in-repo Agent Factory guidance.

## Context

docs/strategies/atelier-rebuild.md; docs/adr/0019-verified-repo-native-tracker.md.

## Close Reason

Current product and architecture guidance is independently reviewed; documented commands and hook examples pass against the integrated production binary.
