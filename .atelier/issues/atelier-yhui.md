---
created_at: "2026-06-19T22:42:56.463983825+00:00"
id: "atelier-yhui"
issue_type: "epic"
labels:
- "validators"
- "workflow-policy"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kmmv"
  - kind: "issue"
    id: "atelier-qx40"
  children:
  - kind: "issue"
    id: "atelier-ee4u"
  - kind: "issue"
    id: "atelier-v4ah"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Namespace validators and fix semantics"
updated_at: "2026-06-20T01:08:22.638624565+00:00"
---

## Description

Refactor workflow validators to namespaced names and fix validator semantics that currently encode status or issue-type magic.

## Outcome

- Validator names read like workflow policy rather than Rust function names.
- Validators remain read-only and do not perform transition actions.

## Evidence

- Workflow policy rejects old flat validator names.
- `review.complete` checks actual configured review artifact completion instead of issue status category.
- Validator docs and tests use namespaced names consistently.
