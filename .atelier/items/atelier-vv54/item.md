---
schema: atelier.item
schema_version: 3
id: atelier-vv54
title: Show malformed Responses lines in the Desk's list of responses
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T10:17:47.771576874Z
---

## Goal

Taken from deferred finding atelier-zc9b/1-4861.1, in .atelier/items/atelier-zc9b/reviews/1-4861.md:

> Finding 1: Desk's structured Responses omit malformed lines
> Recommendation: defer
> [`crates/atelier-desk/src/read.rs`](/home/supadava/.t3/worktrees/atelier/t3-33321399-42619aa2a3efec84/crates/atelier-desk/src/read.rs) now feeds `record.responses()`, which keeps only lines that parse, into [`desk/src/pages/Item.tsx`](/home/supadava/.t3/worktrees/atelier/t3-33321399-42619aa2a3efec84/desk/src/pages/Item.tsx). A malformed `not built:` therefore drops out of "Responses to findings" and its tally. The raw line still sits in the Report markdown and under "The whole record". CLI `status` / `check` / `submit` / `verdict` name the line, and `merge` cannot skip it, so this does not open a way past the gate. The Report already says the Desk page was not driven and has no test.

## Done when

- [ ] An item's page on the Desk shows a malformed line under `### Responses` in "Responses to findings", marked as not read, with the syntax expected, and its tally does not hide it.
- [ ] A Rust test in `crates/atelier-desk/tests/` and a test in `desk/` cover an item with one valid and one malformed line.

## Scope

In: `crates/atelier-desk/src/{read.rs,api.rs}`, `desk/src/api/`, `desk/src/pages/Item.tsx`, and tests.
