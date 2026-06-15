---
created_at: "2026-06-15T03:32:11.279555374+00:00"
id: "atelier-larc"
issue_type: "bug"
labels:
- "cleanup"
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t35w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T05:55:36.381956360+00:00"
status: "done"
title: "Remove hidden issue claim system"
updated_at: "2026-06-15T05:55:36.381956360+00:00"
---

## Description

The hidden issue claim path is still implemented even though the product model
uses active local work instead of a durable claim/assignment system.
`atelier issue update <id> --claim` is hidden from help but still mutates
canonical issue labels by adding `assignee:<actor>` and records
assignee/claimed activity.

## Outcome

- `atelier issue update` no longer accepts or dispatches `--claim`.
- Claim-specific code paths that add `assignee:*` labels or `Claimed by ...`
  notes are removed.
- Tests that currently exercise hidden `--claim` are deleted or rewritten to
  assert rejection.
- Docs/spec references that describe claim as an active Atelier command are
  removed or marked historical only.

## Evidence

- Command output: `atelier issue update <id> --claim` rejects the flag as
  unsupported.
- Help text: `atelier issue update --help` contains no `--claim`.
- Command output: `rg "claim|Claimed by|assignee:" src tests docs` shows no
  active claim dispatch or claim-specific mutation code.
- Tests/lint/export checks pass.
