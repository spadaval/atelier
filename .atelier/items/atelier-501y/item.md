---
schema: atelier.item
schema_version: 3
id: atelier-501y
title: Show the item's title, Done when and mission when start succeeds
status: draft
mission: atelier-4pzj
labels:
- cli-output
- start
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.967000630Z
---

## Goal

A successful `atelier start` (crates/atelier-cli/src/commands/start.rs) prints the Git command, `Recorded workflow_branch on <id>: <branch> (base main)`, `Status`, `Branch`, `Base` and the static line "Commit the record changes with the work they describe." It never says what the work is: no title, Done when or mission, though implementing and ticking every Done when line is the next step. `-q` has no defined output. Agents cut out the line they need: `atelier start X 2>&1 | grep Branch` ran 29 times and `| tail -N` 43 times.

## Done when

- [ ] Success output starts `Started atelier-f0rr "Title" on atelier-f0rr (from master)`, followed by the Done when text (up to about 6 lines), then `Mission: atelier-1w8c "…"` (direct or via parent), then the existing needs-human notes.
- [ ] `atelier -q start <id>` prints only the branch name.
- [ ] CLI tests cover the default output, a child of a parent, and `-q`.

## Context

Dropped in decision atelier-mzpn: a warning when the recorded base differs from the policy base, since atelier-md0v reads the base from the policy everywhere and `start --base` is hidden and ignored.
