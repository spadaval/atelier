---
schema: atelier.item
schema_version: 3
id: atelier-xiiw
title: Say when a catch-up merge's base moved, not that imported statuses changed as no action does
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T03:17:25.862353171Z
---
## Goal

A catch-up merge that was started before the base moved is refused with a
message that blames the wrong thing. On 2026-11-18, atelier-1tkg merged
`origin/master` in and resolved a README conflict. While it did, another
item merged, so `MERGE_HEAD` was no longer the base's tip. The pre-commit
hook then said that atelier-aq82 and atelier-z8rg changed status "as no
action does" and told me to put their statuses back. Doing that would have
been wrong: those records came from the base unchanged. The real cause,
that the merge no longer comes from the base's tip (`verified_base_import`
in `crates/atelier-cli/src/check.rs`), was nowhere in the message.

## Done when

- [ ] When a staged status change would be accepted as a base import except
  that `MERGE_HEAD` is not the configured base's tip, local or its forge
  remote's tracking ref, the refusal says so. It names the merged commit
  and the base's tip now, and says to abort and merge the base again
  (`git merge --abort`, fetch, `git merge <base>`). It does not tell the
  author to put those records' statuses back.
- [ ] A test in `crates/atelier-cli/tests/base_imports.rs` moves the base
  after the merge starts and checks the message.

## Scope

The pre-commit refusal's wording in `crates/atelier-cli/src/check.rs`, and
its test. The rule itself stays as atelier-t83m made it.
