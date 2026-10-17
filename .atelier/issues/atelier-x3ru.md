---
created_at: "2026-10-17T01:10:57.968343817+00:00"
id: "atelier-x3ru"
issue_type: "bug"
labels:
- "bundle"
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Name bundle entries by client_ref in bundle apply graph errors"
updated_at: "2026-10-17T01:10:57.968343817+00:00"
---

## Goal

Staged-graph errors print IDs allocated during staging and then discarded, and never map them back to the bundle. Excerpt from a bundle whose only entry is client_ref `x`:

```
Error: Bundle apply rejected the staged canonical state: workflow_mission_dependency_bypass: mission atelier-fz5l has executable status 'in_progress' despite incomplete dependency path(s): atelier-fz5l -> atelier-ueda
```

`atelier-ueda` exists nowhere and the author cannot tell it was `x`. The cycle error (`bail!` at crates/atelier-records/src/lib.rs:440, `Adding this dependency would create a circular dependency chain`) names no records at all.

## Done when

- A staged failure prints each bundle-created record as its client_ref, e.g. `atelier-fz5l -> x (new)`, never a discarded staging ID.
- A cycle error lists the cycle, e.g. `cycle: a -> b -> a (blocked_by)`.
- Tests cover both messages.

## Context

Error wrapping at crates/atelier-cli/src/commands/bundle.rs:728; ID allocation in apply_bundle_to_state (:810).
