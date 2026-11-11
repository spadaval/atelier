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
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Name bundle entries by client_ref in graph errors, and the issues in a cycle"
updated_at: "2026-11-07T21:03:30.990693392+00:00"
---

## Goal

Two items blocked by each other pass `atelier check` (`Every record is valid`), and `list --blocked` shows each waiting on the other with no word of a cycle (checked 2026-11-11 with the new binary in a converted scratch clone). Neither can ever start, and nothing says why.

## Done when

- [ ] `check` reports a `blocked_by` cycle, naming the items in it, e.g. `cycle: atelier-a -> atelier-b -> atelier-a (blocked_by)`.
- [ ] A test covers a two-item and a three-item cycle.

## Context

Error wrapping in crates/atelier-cli/src/commands/bundle.rs; ID allocation in apply_bundle_to_state. The record_index cycle message joined this item from atelier-t18i in decision atelier-mzpn.

Rewritten for the new tool in the cutover sort (atelier-5zre): `bundle` and its staging IDs are gone, so only the cycle error survives; the new records store each blocker as `blocked_by` on the blocked item, and the new `check` accepts a cycle.
