---
created_at: "2026-06-15T03:54:39.200655416+00:00"
id: "atelier-t35w"
issue_type: "validation"
labels:
- "state-model"
- "validation"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-lu10"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "blocked"
title: "Validate active issue and claim removal"
updated_at: "2026-06-15T06:21:19.572662291+00:00"
---

## Description

Independently validate that the active issue and legacy claim systems have been removed without losing current-work orientation or breaking normal issue workflow. This item validates the epic claims after implementation, not during implementation.

## Outcome

- Current work is recoverable from canonical Markdown after runtime/cache deletion and rebuild.
- Removed command surfaces are absent from help and rejected when invoked.
- Hidden claim behavior no longer mutates labels, assignee state, activity, or issue metadata.
- Normal issue workflow still supports creating, starting, reviewing/validating where configured, recording evidence, and closing work.

## Evidence

- Independent validation transcript covers cache deletion/rebuild, `atelier status`, `atelier start`, removed command rejection, and hidden `--claim` rejection.
- Targeted source search transcript shows no active dispatch for runtime active issue or claim mutation behavior.
- Focused integration tests, `atelier lint atelier-t35w`, `atelier export --check`, and `git diff --check` pass.
- Validation evidence record is attached to this issue and referenced by epic closeout.
