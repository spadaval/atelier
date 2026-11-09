---
created_at: "2026-11-09T14:30:20.861505709+00:00"
id: "atelier-t7pb"
issue_type: "task"
labels:
- "fleet-dashboard"
workflow_branch:
  branch_base: master
  work_branch: atelier-t7pb
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make atelier debrief one history walk instead of one per item: 7.4 s to under 1.5 s on mission atelier-k8h8"
updated_at: "2026-11-09T15:34:36.240109563+00:00"
---

## Goal

Make `atelier debrief` fast: on this repository `atelier debrief atelier-k8h8 -q` (45 items) takes 7.4 s, and nearly all of it is 45 separate walks of the whole history. `done_when_edits` (`crates/atelier-app/src/record_history.rs:490`) runs one `git log --reverse --full-history --format="%H %P" <head> -- .atelier/issues/<id>.md` per item, about 0.18 s each over 4,086 commits. One walk over `.atelier/issues/` with `--name-only`, bucketed by path, takes 0.28 s for the whole mission. The site serves debriefs from this host (mission atelier-yfqu), and `review request` and `merge --check` call the same function, so all three get faster.

## Done when

- [x] `done_when_edits` reads the history of every id it is given with one `git log` walk, whatever the number of ids, and keeps its signature and result. A test with a fixture of several items counts the `git log` runs (through the existing read context or a test hook) and finds one.
- [x] On every mission in this repository, `atelier debrief <mission> -q` and `atelier debrief <mission>` print the same output from the new build as from a build of `master` before the change, and `atelier merge --check` and `review request`'s Done when section agree likewise on this repository's open branches. The Report names the command that compared them and the count.
- [x] On this host, `atelier debrief atelier-k8h8 -q` from this repository takes under 1.5 s wall with a warm page cache, measured with `/usr/bin/time -v` before and after; the Report records both numbers and the new top cost if one remains.

## Scope

The per-item walk in `done_when_edits` and any other per-item `git log` the debrief runs that the same measurement shows costs more than 0.5 s in total. No caching layer and no commit-graph setup; the site's cache is atelier-zpmb's.

## Context

Found on 2026-11-09 by tracing `atelier debrief atelier-k8h8 -q` with `strace -f -e trace=execve`: 88 git processes ran, 45 of them this walk. With a commit-graph written with `--changed-paths` the per-file walk was about 5x faster in a scratch clone, which says the cost is history traversal, not record parsing. The other per-item walk (`--no-merges` at `record_history.rs:432`) costs about 0.05 s per call.

## Notes

- 2026-11-09 [decision] The walk takes the .atelier/issues/ directory as its one pathspec and keeps the given records' paths in process, with --diff-merges=separate --root --no-renames so a record is listed wherever its file differs from any parent, as the per-record --full-history walk lists it.
  Reason: Git's walk cost grows with the number of pathspecs: 29 file pathspecs took 1.2 s, the directory 0.32 s. Separate merge diffs reproduce --full-history's selection exactly, so no argument about which commits can matter is needed.
  Rejected: One pathspec per record: 2.0 s for mission atelier-k8h8, only 3.7x faster
  Rejected: Reuse path_log's RecordGraph: debrief does not load it, and its load diffs every commit against every parent over reviews too

## Report

All three Done when lines hold. `done_when_edits` now reads every item's history from one `git log` over `.atelier/issues/`, bucketed by path, with its signature and result unchanged. The new test `record_history::tests::done_when_edits_reads_every_item_from_one_git_log_walk` passes: on three items, one edited on a merged branch, it counts one `git log` run (`ReadContext::logs_run`), gets the expected edits, and finds each item's share of the walk equal to its own per-record walk. Doing the walk per item made it fail with 4 runs. Comparing the master build with the new one (`diff -r` over saved output and exit status) found all 56 outputs identical: `debrief` and `debrief -q` on all 22 missions (44), and `merge --check` and `review request` on the 3 open branches and 3 merged branches recreated at their pre-merge tips (12, with 4 Done when edits among them). `/usr/bin/time -v atelier debrief atelier-k8h8 -q` went from 7.80 s to 1.04 s wall. The top remaining cost is `issue_histories`' first-parent walk at 0.38 s, filed as atelier-lr08. Receipts: atelier-3t2x (test), atelier-92gg (comparison), atelier-nsz3 (timing).

### Deviations

The master binary was built in this worktree's own `target/` before any edit and copied to `~/.cache/atelier-t7pb-base/`, instead of building it in a separate scratch worktree. It is the same code, and this avoided a second full build.

### Surprises

Most of the cost was Git matching many pathspecs, not walking history. Merging the walks with one file pathspec per item only got the debrief to 2.7 s, because that one walk took 2.0 s. With the directory as the pathspec, the walk takes 0.31 s.

### Decisions

The directory pathspec and `--diff-merges=separate`: see the decision note.
