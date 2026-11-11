---
schema: atelier.item
schema_version: 3
id: atelier-lr08
title: 'Give issue_histories one directory pathspec: its walk is 0.38 s of the debrief''s 1.04 s on mission atelier-k8h8'
status: draft
mission: null
labels:
- fleet-dashboard
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-09T15:33:02.846790859Z
---

## Goal

`issue_histories` (`crates/atelier-app/src/record_history.rs`, used by `mission_timeline`) runs `git log --first-parent --diff-merges=first-parent --name-only` with one pathspec per record, 45 on mission atelier-k8h8. Git's cost grows with the pathspec count: after atelier-t7pb it is the debrief's top cost, 0.38 s of 1.04 s wall. atelier-t7pb found the same for the full-history walk, where 29 file pathspecs took 1.2 s and the `.atelier/issues/` directory 0.32 s, with output bucketed by path in process.

## Done when

- [ ] `issue_histories` walks `.atelier/issues/` with one pathspec and keeps only the given records' paths, with its result unchanged: `atelier debrief <mission>` and `-q` print the same on every mission here before and after.
- [ ] `/usr/bin/time -v atelier debrief atelier-k8h8 -q` before and after are in the Report.

## Context

Measured with `GIT_TRACE2_EVENT` on 2026-11-09 after atelier-t7pb. Low priority: the debrief is already under the 1.5 s target. The `fork`/entry-commit path in `walk` passes the same pathspecs to `git diff`; it may want the same treatment.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-t7pb (Make atelier debrief one history walk instead of one per item: 7.4 s to under 1.5 s on mission atelier-k8h8)
