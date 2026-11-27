---
schema: atelier.item
schema_version: 3
id: atelier-jk0s
title: Allocate an ID that no item added on another unmerged branch already has
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T18:46:12.383902428Z
---

## Goal

`docs/design.md` says `new` picks an ID that no record has on any branch
the reader reads. `Repo::new_id` reads the checkout, the base's tip and
the names of branches, and not the items that other unmerged branches
add. Since atelier-u6oy items are filed on every branch, so two can be
given one ID and conflict when the second merges. With four base-36
characters that is rare, and the document is wrong either way. Found by
reading, by atelier-u6oy's implementer.

## Done when

- [ ] `new` and `start --new` never pick the ID of an item that an
  unmerged named branch adds, local or fetched, and a test holds it; or
  `docs/design.md` says what is read instead and why that is enough.

## Scope

`Repo::new_id` and that paragraph of `docs/design.md`.
