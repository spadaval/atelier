---
created_at: "2026-11-09T14:30:20.861505709+00:00"
id: "atelier-t7pb"
issue_type: "task"
labels:
- "fleet-dashboard"
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
status: "todo"
title: "Make atelier debrief one history walk instead of one per item: 7.4 s to under 1.5 s on mission atelier-k8h8"
updated_at: "2026-11-09T14:30:20.861505709+00:00"
---

## Goal

Make `atelier debrief` fast: on this repository `atelier debrief atelier-k8h8 -q` (45 items) takes 7.4 s, and nearly all of it is 45 separate walks of the whole history. `done_when_edits` (`crates/atelier-app/src/record_history.rs:490`) runs one `git log --reverse --full-history --format="%H %P" <head> -- .atelier/issues/<id>.md` per item, about 0.18 s each over 4,086 commits. One walk over `.atelier/issues/` with `--name-only`, bucketed by path, takes 0.28 s for the whole mission. The site serves debriefs from this host (mission atelier-yfqu), and `review request` and `merge --check` call the same function, so all three get faster.

## Done when

- [ ] `done_when_edits` reads the history of every id it is given with one `git log` walk, whatever the number of ids, and keeps its signature and result. A test with a fixture of several items counts the `git log` runs (through the existing read context or a test hook) and finds one.
- [ ] On every mission in this repository, `atelier debrief <mission> -q` and `atelier debrief <mission>` print the same output from the new build as from a build of `master` before the change, and `atelier merge --check` and `review request`'s Done when section agree likewise on this repository's open branches. The Report names the command that compared them and the count.
- [ ] On this host, `atelier debrief atelier-k8h8 -q` from this repository takes under 1.5 s wall with a warm page cache, measured with `/usr/bin/time -v` before and after; the Report records both numbers and the new top cost if one remains.

## Scope

The per-item walk in `done_when_edits` and any other per-item `git log` the debrief runs that the same measurement shows costs more than 0.5 s in total. No caching layer and no commit-graph setup; the site's cache is atelier-zpmb's.

## Context

Found on 2026-11-09 by tracing `atelier debrief atelier-k8h8 -q` with `strace -f -e trace=execve`: 88 git processes ran, 45 of them this walk. With a commit-graph written with `--changed-paths` the per-file walk was about 5x faster in a scratch clone, which says the cost is history traversal, not record parsing. The other per-item walk (`--no-merges` at `record_history.rs:432`) costs about 0.05 s per call.
