---
schema: atelier.item
schema_version: 3
id: atelier-x3ru
title: Name bundle entries by client_ref in graph errors, and the issues in a cycle
status: draft
mission: atelier-uf97
labels:
- bundle
- cli-output
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.968343817Z
---

## Goal

Staged-graph errors print IDs allocated during staging and then discarded, and never map them back to the bundle. Excerpt from a bundle whose only entry is client_ref `x`:

```
Error: Bundle apply rejected the staged canonical state: workflow_mission_dependency_bypass: mission atelier-fz5l has executable status 'in_progress' despite incomplete dependency path(s): atelier-fz5l -> atelier-ueda
```

`atelier-ueda` exists nowhere and the author cannot tell it was `x`. The cycle error (`bail!` in crates/atelier-records/src/lib.rs, `Adding this dependency would create a circular dependency chain`) names no records at all. The graph's own cycle error, from loading records or `atelier check` (crates/atelier-app/src/record_index.rs), says `relationships.blocks contains a cycle`, but new links are stored as `blocked_by` in `relates`, so a reader looking in `blocks` finds nothing (atelier-t18i, finding atelier-472z).

## Done when

- [ ] A staged failure prints each bundle-created record as its client_ref, e.g. `atelier-fz5l -> x (new)`, never a discarded staging ID.
- [ ] Both cycle errors list the cycle and the link role, not a storage field, e.g. `cycle: a -> b -> a (blocked_by)`.
- [ ] Tests cover each message.

## Context

Error wrapping in crates/atelier-cli/src/commands/bundle.rs; ID allocation in apply_bundle_to_state. The record_index cycle message joined this item from atelier-t18i in decision atelier-mzpn.
