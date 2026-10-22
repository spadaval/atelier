---
created_at: "2026-10-17T06:11:21.436442713+00:00"
id: "atelier-rcnk"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T20:43:20.103489695+00:00"
status: "done"
title: "Clean up merged-tree checkouts after an interrupted merge"
updated_at: "2026-10-22T20:54:58.159242249+00:00"
---

## Goal

Under `merge_strategy: merge_commit`, `atelier merge` runs `merge_checks` in a temporary detached worktree under `<git-common-dir>/atelier-merge-*/tree`, removed by `Drop`. If merge is interrupted (Ctrl-C, kill during long checks), the worktree stays registered and its directory stays, so `git worktree prune` does not remove it. Each run also starts with no build cache, so a cargo check rebuilds from scratch. Found in review of atelier-ydv0 (finding atelier-gle3).

## Done when

- A later merge, `atelier prune`, or `doctor` finds and removes leftover merged-tree checkouts, shown by a test that leaves one behind.
- Whether merged-tree checks may reuse a build cache is decided and documented.

## Notes

- 2026-10-22 [decision] Merged-tree checks reuse no build cache: each builds in its own checkout's target directory, removed with it; a policy may add a content-keyed compiler cache such as sccache inside its check command
  Reason: A shared cargo target directory has run another branch's binary in tests (atelier-isyf), and a cold build is the cost of proving the merged tree itself. A compiler cache keyed by inputs cannot serve a stale result.
  Rejected: A repo-wide shared target directory across merges: stale or foreign binaries; a per-base cache that Atelier manages: new state to expire and repair
- 2026-10-22 [decision] Leftover checkouts are found by a free lock on atelier-merge-*/lock and swept at the start of the next merge; prune and doctor do not sweep them
  Reason: A killed merge cannot run Drop, but the OS releases its lock, so a free lock distinguishes a leftover from a concurrent merge in another worktree, which pid files cannot do safely. The Done when asks for any of merge, prune or doctor; merge is the command that creates them.
  Rejected: Pid files: pid reuse and no atomic claim; deleting every atelier-merge-* directory: would break a concurrent live merge
- 2026-10-22 [decision] The sweep treats a directory already gone as removed and reports other removal failures as warnings; a live checkout from a binary older than the lock gets no protection
  Reason: A sweep must not fail a merge over housekeeping. A pre-lock checkout (no lock file) could be swept by a new binary, but every session here runs the reinstalled binary after merge, so the window does not occur.
  Rejected: Treating a missing lock file as live: strands the leftovers of a kill between mkdir and lock

## Close Reason

The next `atelier merge` under `merge_commit` removes merged-tree checkouts an interrupted merge left behind (registered worktree and directory), keeps one a live merge still holds, and a test kills atelier during its checks to prove it. Merged-tree checks reuse no build cache; this is decided and documented in docs/product/reviews-and-merge.md.

### Deviations
Only merge sweeps; `atelier prune` and `doctor` do not, since the Done when allows any of the three.

### Decisions
Two decision notes on this item: no build-cache reuse, and lock-based leftover detection swept at merge start.

### Surprises
A pid file cannot tell a leftover from a concurrent merge in another worktree safely, so the checkout directory carries a lock the OS drops on death. Std `File::lock` suffices; no new dependency.

### Open items
None.
