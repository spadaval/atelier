---
created_at: "2026-10-17T01:10:57.979644079+00:00"
id: "atelier-7ts2"
issue_type: "bug"
labels:
- "cli-output"
- "land"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T15:49:55.911470310+00:00"
status: "done"
title: "Replace land_completion_dirty's impossible advice with the dirty paths"
updated_at: "2026-10-23T15:49:55.911470310+00:00"
---

## Goal

On a done owner branch with one modified file, `atelier land` prints:
```
Error: land_completion_dirty: a completed owner requires a clean checkout for current proof/review; commit or hand off the work as active
```
(crates/atelier-cli/src/commands/session.rs:463). The policy has no transition out of `done`: `atelier issue transition atelier-43m1` returns `Issue atelier-43m1 has no configured transitions from status 'done'`. So "hand off the work as active" cannot be done. The message also does not name the dirty paths, which `initial_paths` already holds.

## Done when

- The message lists the dirty paths (first 5, then a count) and says to commit them (`git add … && git commit`) or discard them (`git restore …`).
- If reopening a done owner is meant to be possible, the message names the real route. Otherwise the advice is dropped.
- A test asserts the paths appear.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed (session.rs:354). Factor the 5-plus-count path lister that exists three times in session.rs (:409-418, :437-446, :174-183).
- 2026-10-23 [decision] land_completion_dirty offers commit (git add -A && git commit -m "…") or set aside (git stash push --include-untracked), not git restore.
  Reason: restore cannot remove untracked files and discards work; stash is recoverable and runs verbatim. Reopening a done owner is impossible (no transition leaves done), so that advice is dropped.

## Close Reason

`land_completion_dirty` now names the done owner's uncommitted paths (first five, then a count) and offers `git add -A && git commit -m "…"` or `git stash push --include-untracked`, both run verbatim by the fix helper; the impossible "hand off the work as active" advice is gone, and the gates still run beside it.

### Deviations
"Discard with git restore" became "set aside with git stash push --include-untracked".

### Decisions
See the decision note on this item. The 5-plus-count lister is one `first_five` helper in atelier-core, used for all three session.rs lists and the record list.

### Surprises
None.

### Open items
Paths hidden by Git index flags are named but neither offered command captures them; proposed as a follow-up in the handoff, not filed.
