---
schema: atelier.item
schema_version: 3
id: atelier-6hb2
title: 'Release 0.6.0: the branch model, atelier log, and merging through the forge''s pull request'
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- b4aa5610-b69d-4bf1-b9a2-3b7ca76739c4
ready: null
trivial: null
caused_by: null
created_at: 2026-11-14T06:12:41.060460543Z
---

## Goal

Publish a release that matches what agents on this host already run. The
latest release, v0.5.0, was tagged on 2026-11-11 before the branch model,
`atelier log` and merging through the forge's pull request, so an install
from the release lacks them. This item sets the version to 0.6.0 and writes
its changelog; once it merges, `v0.6.0` is tagged on the merge commit and the
release workflow builds it.

## Done when

- [x] The workspace version in `Cargo.toml` is 0.6.0, and `Cargo.lock` agrees.
- [x] `CHANGELOG.md` has a 0.6.0 entry that says what changed for a user since
  0.5.0, and whether a 0.5 repository needs anything to move.
- [x] Nothing else in the tree states the current version as 0.5.0; history
  in records and the design record is left as written.

## Scope

The version, the changelog and one line of the cutover README. Tagging and
the release build happen after the merge, outside the branch.

## Report

The version is 0.6.0 in `Cargo.toml` and in the five workspace packages in
`Cargo.lock` (`cargo update -w --offline`). The changelog entry was written
from the merges since v0.5.0 and checked against the commands' help in
`crates/atelier-cli/src/main.rs` and the pre-push hook in `check.rs`. 0.6.0
still reads a record in the old `draft` mission status, and the hooks are
`exec atelier check --hook <name>`, so a 0.5 repository needs no migration
or new `setup`.

`scripts/cutover/README.md` said a finished cutover prints `atelier 0.5.0`;
it now says 0.5.0 or later. `cutover.sh` checks only for the old 0.4 binary
before it starts, so nothing else there names the version.

Minor, not semver-major: the project is pre-1.0 and 0.5 records read
unchanged, but commands were removed (`escalate`, `decide`, `new
--mission`), so a patch release would undersell it.

### Responses

- 1-amhk.1: fixed in 342759dbe. The changelog has an Upgrading from 0.5 section: a `ready` or `open` mission carries on, a `draft` one continues on a branch named for its ID through `submit` and two plan approvals, and `ready` takes items only.
