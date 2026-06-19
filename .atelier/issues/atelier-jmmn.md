---
created_at: "2026-06-17T20:03:36.404673836+00:00"
id: "atelier-jmmn"
issue_type: "task"
labels:
- "bundle"
- "cli"
- "implementation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-mrj5"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T22:05:53.505825161+00:00"
status: "done"
title: "Move graph creation off plan apply and into bundle commands"
updated_at: "2026-06-17T22:05:53.505825161+00:00"
---

## Description

Move declarative graph creation out of `atelier plan apply` and into
`atelier bundle`. Bundle apply is not a plan record operation; it is an advanced
multi-record graph mutation command that consumes an authored file.

## Outcome

- Public help exposes `atelier bundle preview <file>` and
  `atelier bundle apply <file> --yes`, and no longer
  routes users through `atelier plan apply`.
- The implementation module naming and tests match the new product concept.
- Backward-compatible shims are not added unless a human explicitly asks for
  them.

## Evidence

- Help transcript proves the bundle command is discoverable and the old
  `plan apply` surface is absent or rejected.
- Focused CLI tests cover the renamed command and removed command surface.
