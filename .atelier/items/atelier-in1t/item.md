---
schema: atelier.item
schema_version: 3
id: atelier-in1t
title: The review brief's whole-change diff reads the local base, not the forge's
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
created_at: 2026-11-22T10:17:47.554251170Z
---

## Goal

The review brief prints its whole-change diff against the local base, `git
diff master...<commit>`, where the rest of the tool reads the forge's base.
In a clone whose local `master` is behind `origin/master`, with
`origin/master` merged into the branch, that diff also holds upstream's
changes, and the reviewer reads work that is not the item's. The delta diff
`atelier-h6ij` added reads the forge's base already. Found by
`atelier-h6ij`'s implementer.

## Done when

- [ ] The brief's whole-change diff names the base the tool judges by: the remote-tracking base where a remote is configured.
- [ ] A test in `crates/atelier-cli/tests/` makes a clone with a stale local base and proves the brief's diff holds only the branch's changes.

## Scope

In: `crates/atelier-cli/src/review.rs`, `crates/atelier-cli/src/output.rs`, and tests.
