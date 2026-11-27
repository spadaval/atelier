---
schema: atelier.item
schema_version: 3
id: atelier-fbte
title: Set the version to 0.8.0 and tell a host what changes when it updates
status: open
mission: atelier-oqsz
labels: []
priority: medium
blocked_by: []
implementers: []
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

- [ ] The workspace version is 0.8.0 in `Cargo.toml`, and `Cargo.lock`
  agrees.
- [ ] `CHANGELOG.md` has a 0.8.0 entry in the form of the 0.7.0 one,
  written from `git log v0.7.0..origin/master --first-parent`. It says
  what changed for agents using `atelier` and what a host must do when it
  updates, and each claim matches the merged change it describes.
- [ ] The entry says that no record changes form, that a reader of plain
  `atelier log` must pass `--all`, that a vendored copy of the skill is
  refreshed with the binary, and how a failed merge check is passed now.

## Scope

The version, the lockfile and the changelog. No tag and no release build:
those are the human's to ask for.
