---
schema: atelier.item
schema_version: 3
id: atelier-fbte
title: Set the version to 0.8.0 and tell a host what changes when it updates
status: approved
mission: atelier-oqsz
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A9a9dbac4-2254-4e6f-a69b-b8075e32a1ff%3Adelegate-task%3Aoqsz-impl-fbte
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T09:13:19.110832055Z
---

## Goal

Mission `atelier-oqsz` changed what `status`, `merge` and `log` print and
do, and what the review briefs say, and the tree still calls itself 0.7.0.
A host cannot tell a new binary from the installed one, and two of the
changes break something a host has: a script or a vendored skill that
reads plain `atelier log` now gets the 30 newest actions, and a lead's
reason for one refusal no longer carries a merge past a failed merge
check. The plan had the lead bump the version in a trivial merge. A
changelog entry is prose, which is not trivial, so it is this item.

## Done when

- [x] The workspace version is 0.8.0 in `Cargo.toml`, and `Cargo.lock`
  agrees.
- [x] `CHANGELOG.md` has a 0.8.0 entry in the form of the 0.7.0 one,
  written from `git log v0.7.0..origin/master --first-parent`. It says
  what changed for agents using `atelier` and what a host must do when it
  updates, and each claim matches the merged change it describes.
- [x] The entry says that no record changes form, that a reader of plain
  `atelier log` must pass `--all`, that a vendored copy of the skill is
  refreshed with the binary, and how a failed merge check is passed now.

## Scope

The version, the lockfile and the changelog. No tag and no release build:
those are the human's to ask for.

## Report

The tree now calls itself 0.8.0, and `CHANGELOG.md` has a 0.8.0 entry in
the 0.7.0 entry's form, written from `git log v0.7.0..origin/master
--first-parent` and the Reports of the items it merged. Commit bd767b18a.

- `Cargo.toml`'s workspace version is 0.8.0. `Cargo.lock` gives 0.8.0 for
  `atelier-cli` and `atelier-core`; `atelier-desk` keeps its own 0.0.0.
  `cargo metadata --locked` accepts the lock.
- The entry covers the whole range since v0.7.0, not only `atelier-oqsz`:
  open items (`atelier-v06d`), filing on any branch (`atelier-u6oy`),
  branch cleanup (`atelier-gqtw`), handoff fixes (`atelier-zc9b`), round
  limits (`atelier-h6ij`, `atelier-qut2`), `not built` (`atelier-hlsw`),
  `Atelier-Skip-By` (`atelier-nv22`), `plan`/`plan-merge` (`atelier-js4a`),
  the lead in charge (`atelier-mtzj`), and the mission's `status`, pace,
  `log` and merge-check changes. Eval-only merges are left out, since they
  change nothing a host or an agent sees.
- Under "Updating a host" it says that plain `atelier log` readers must
  pass `--all`, that a vendored skill (as under `.agents/skills/agent-factory`)
  is refreshed with the binary, and that a failed merge check is passed
  with `ATELIER_SKIP="<check>: <why it may fail>" atelier merge`.

One premise needed qualifying. "No record changes form" holds for this
mission's items and for reading: 0.8.0 reads every 0.7.0 record as it is,
with `draft` and `ready` read as `open`. Since v0.7.0, though, the tree
writes two spellings a 0.7.0 binary refuses, `status: open`
(`atelier-v06d`) and `<finding>: not built: <reason>` (`atelier-hlsw`; I
checked that v0.7.0's `records/body.rs` parses only `fixed in` and
`rebutted:`). The entry says both, and tells a host to update the binary,
the Desk, the extension and any CI pinned to v0.7.0 before a repository
holds an open item, which is `atelier-pv58`'s case.

Not done, as Scope says: no tag and no release build. Nothing was built
or tested beyond `cargo metadata --locked` and `git diff --check`; CI
builds the version on the pull request.
