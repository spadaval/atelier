---
schema: atelier.item
schema_version: 3
id: atelier-s2ho
title: Count a CI run for a head that differs from it only by newly filed items
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T18:46:12.708767500Z
---

## Goal

An item filed after CI ran makes `merge` wait for CI again, up to 9
minutes, though it changes no code. A run counts for the head only where
the two differ inside the item's own directory (`merge.rs`, `counts`).
atelier-u6oy made such a commit leave the approval standing and left CI's
rule as it was, so filing still has that cost at the moment an approved
item is about to merge.

## Done when

- [ ] A CI run counts for a head that differs from its commit only inside
  the item's directory and by newly filed items, by the same test as the
  approval uses, and a test holds it; or this is closed with the reason
  the wait is wanted.
- [ ] `docs/design.md` and `docs/product.md` say which runs count.

## Scope

`merge`'s choice of the CI runs that count.
