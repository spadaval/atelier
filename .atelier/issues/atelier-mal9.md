---
created_at: "2026-11-10T00:56:06.471222574+00:00"
id: "atelier-mal9"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-mal9
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
status: "in_progress"
title: "Read, write and validate item, review and config records in the new layout"
updated_at: "2026-11-10T14:58:34.868784826+00:00"
---

## Goal

Build the record store the new commands stand on: `items/<id>/item.md`, `reviews/<round>-<suffix>.md` and `config.toml` at schema version 3, as the architecture specification's Records section defines them. This is the first code of the full rewrite, so it also creates the new workspace, `next/`, with the crates `atelier-core` and `atelier-records` at `next/crates/<name>/`.

## Done when

- [ ] Unit tests parse and write each record kind and get the same file back, with the Markdown body preserved byte for byte.
- [ ] Validation rejects each of these with a message naming the file and the field: an unknown status; an item with no Goal or no Done when section; a finding with no Recommendation line; a review whose verdict is approve while it holds an open fix-now finding; a `blocked_by` that names no item.
- [ ] An item's `implementers` and a review's `reviewers` are lists; unit tests cover appending a thread and reading the last entry as the holder.
- [ ] A finding's name is its review's file stem and its number, as in `2-m2bn.1`; unit tests parse `Was:`, Responses lines and a Decisions entry's `Finding:` in that form, and reject a name no review has.
- [ ] An item record carries an optional `caused_by`, holding an item ID or a commit hash. It is checked for form only, since the item that caused a bug may have left the tree.
- [ ] Unit tests cover when a deferred finding is settled: it is unsettled until its text has a `Taken: <item> by <thread>` or a `Dropped by <thread>: <reason>` line, and both lines parse and write back unchanged.
- [ ] Unit tests cover when a fix-now finding is open: it stays open until a later round marks it fixed or withdrawn, or a Decisions entry naming it has an outcome.
- [ ] Creating a review for round 2 writes `reviews/2-<suffix>.md` with four random characters, and two reviews created for the same round have different names.
- [ ] `config.toml` parsing covers the base branch, the merge strategy, merge checks, and a forge with and without `ci_workflow`; an unknown key is an error that names it.
- [ ] `next/` is a Cargo workspace of its own with its own lock file, and `next/target/` is ignored by Git. The CI job that builds and tests `next/` deletes the root `crates/` directory before it builds, so the new workspace cannot build from the old one by any route. The Report shows that job's steps, run as written in a scratch clone, failing on a throwaway commit that adds a path dependency on an old crate.
- [ ] `.github/workflows/ci.yml` has jobs that build and test `next/`. `scripts/ci-jobs.sh` says which workspace a change needs, the new one when a path under `next/` changes and the old one when a path under `crates/` does, and `scripts/ci-jobs.test.sh` covers both. No file under `crates/` is changed.
- [ ] `scripts/count_non_test_rust.py` takes a directory and counts only under it, so the two workspaces can be reported apart.
- [ ] The Records section of `docs/strategies/redesign-architecture.md` matches what was built; the Report lists each difference corrected there.

## Scope

Item, review and config records. Mission records, and checking that an item's `mission` names one, belong to step B. No CLI. Front-matter splitting is copied from `split_document` in `crates/atelier-records/src/lib.rs` with one change: it keeps the body as the raw slice, where today it strips one newline at each end, so a body with and without those newlines stays distinguishable; a unit test covers both. ID generation is copied from `crates/atelier-records/src/record_id.rs`. Both are trimmed to what the new records use and reviewed as new code. Nothing else is copied; the rest is written new.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records and Crates. The two pieces to copy are in `crates/atelier-records/src/lib.rs` and `crates/atelier-records/src/record_id.rs`. The rule for copying is in the architecture specification's section "The rule for this work". This is the first item of step A in `docs/strategies/redesign-2-core.md`.

## Notes

- 2026-11-10 [decision] The orchestrator reworded the CI Done when line: the Report shows the new job's steps, run as written in a scratch clone, failing on a throwaway commit that adds a path dependency on an old crate (and passing without it), instead of a real CI run.
  Reason: A real CI run needs a hand-pushed throwaway branch on origin; the merge policy forbids pushing branches by hand, and only the owner may delete remote branches.
  Rejected: Push a throwaway branch to origin and let CI fail on it
