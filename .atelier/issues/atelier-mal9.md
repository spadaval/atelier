---
created_at: "2026-11-10T00:56:06.471222574+00:00"
id: "atelier-mal9"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Read, write and validate item, review and config records in the new layout"
updated_at: "2026-11-10T03:58:04.085752421+00:00"
---

## Goal

Build the record store the new commands stand on: `items/<id>/item.md`, `reviews/<round>-<suffix>.md` and `config.toml` at schema version 3, as the architecture specification's Records section defines them, in new modules of `atelier-core` and `atelier-records`.

## Done when

- [ ] Unit tests parse and write each record kind and get the same file back, with the Markdown body preserved byte for byte.
- [ ] Validation rejects each of these with a message naming the file and the field: an unknown status; an item with no Goal or no Done when section; a finding with no Recommendation line; a review whose verdict is approve while it holds an open fix-now finding; a `blocked_by` that names no item.
- [ ] An item's `implementers` and a review's `reviewers` are lists; unit tests cover appending a thread and reading the last entry as the holder.
- [ ] A finding's name is its review's file stem and its number, as in `2-m2bn.1`; unit tests parse `Was:`, Responses lines and a Decisions entry's `Finding:` in that form, and reject a name no review has.
- [ ] An item record carries an optional `caused_by`; validation rejects one that names no item.
- [ ] Unit tests cover when a deferred finding is settled: it is unsettled until its text has a `Taken: <item>` or a `Dropped: <reason>` line, and both lines parse and write back unchanged.
- [ ] Unit tests cover when a fix-now finding is open: it stays open until a later round marks it fixed or withdrawn, or a Decisions entry naming it has an outcome.
- [ ] Creating a review for round 2 writes `reviews/2-<suffix>.md` with four random characters, and two reviews created for the same round have different names.
- [ ] `config.toml` parsing covers the base branch, the merge strategy, merge checks, and a forge with and without `ci_workflow`; an unknown key is an error that names it.
- [ ] The old binary's tests pass unchanged.
- [ ] The Records section of `docs/strategies/redesign-architecture.md` matches what was built; the Report lists each difference corrected there.

## Scope

Item, review and config records. Mission records, and checking that an item's `mission` names one, belong to step B. No CLI. The new modules reuse the existing front-matter splitting and ID generation and do not depend on `atelier-workflow` or on the old issue and review record types; the old code is not changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records and Crates. The existing parser is `crates/atelier-records/src/lib.rs`. This is the first item of step A in `docs/strategies/redesign-2-core.md`.
