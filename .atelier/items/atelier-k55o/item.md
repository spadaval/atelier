---
schema: atelier.item
schema_version: 3
id: atelier-k55o
title: '`ready` accepts an item whose Goal and Done when are empty'
status: draft
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.961551559Z
---

## Goal

`atelier new "x"` writes an item whose Goal, Done when and Scope are empty headings, and `atelier ready <id> --by <name>` then makes it ready as it is (reproduced in a scratch clone on 2026-11-11, binary built from master b4895f93). A ready item with no Done when gives its implementer nothing to tick and its reviewer nothing to judge. Say where the empty item is caught: at `ready`, in `check`, or not at all.

## Done when

- [ ] Provisional: `ready` refuses an item whose Goal or Done when is empty, naming the section, and a test covers it; or the Report says why an empty ready item is allowed.

## Context

Rewritten on 2026-11-11 from the old tool's form of this bug (`issue create` writing "Done when was not specified."), when the backlog was swept after the 0.5 cutover. A draft may stay empty: capture now, refine later.
