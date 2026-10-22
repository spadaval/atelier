---
created_at: "2026-10-17T19:00:36.351224229+00:00"
id: "atelier-s2ei"
issue_type: "feature"
labels:
- "merge"
- "retro-2026-10-17"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-s2ei
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "issue"
    id: "atelier-n642"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add atelier merge --sync to bring the base in, re-prove and report approval survival"
updated_at: "2026-10-22T22:22:21.180488011+00:00"
---

## Goal

Orchestrators hand-built the same macro 44+ times: commit records, `git merge master`, rerun both evidence suites, record evidence, `atelier merge`, `land`, push only on success. One command should do the catch-up step and say whether the approval survived (clean merge) or which conflict-resolved files need re-review.

## Done when

`atelier merge <id> --sync` merges the base into the owner branch, reruns the recorded evidence commands, records their receipts, and either reports the approval as kept (clean merge) or lists the conflict-resolved paths that need a fresh approval, ending in a verdict. Tests cover a clean merge, a conflicted merge, and a failing rerun.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). o3lm: 78 `git merge master` invocations, 52 merge commits, 25 suite reruns labelled 'after merging master'.

## Notes

- 2026-10-22 [decision] On conflict, leave git merge in progress: list conflicted files on stderr, commit nothing, verdict 'merge: refused — 1 check (sync_conflict)'. Running --sync again after the user's resolution commit resumes: it reruns receipts current at the merge commit's first parent
  Reason: The spec says stop with the files listed and nothing committed. Resuming makes conflict resolution part of one flow, and the approval report then lists the hand-resolved paths needing re-review
  Rejected: git merge --abort automatically (loses the user's chance to resolve)
- 2026-10-22 [decision] Rerun only the latest passing command-backed receipt per (scope item, evidence kind, command) that was fresh before the sync; record a new receipt with the same command, kind, target, role and criteria, committed in one 'Record evidence rerun' commit. A failing rerun is recorded and refuses with sync_rerun_failed. Manual receipts that were current are named and left as recorded
  Reason: Receipts store the command they captured, so rerun reproduces them via split_command (inverse of format_command); stale or failed receipts were not proof before the sync, and a failure must stay on the record for the gates
  Rejected: rerunning every command ever recorded; dropping failed reruns
- 2026-10-22 [decision] Approval survival uses review_gate::approval_accepted and a new approval_stale_edge that approval_is_current_to now delegates to. A stale approval is reported with the first failing commit and its hand-resolved or edited paths but does not make the verdict refused
  Reason: One rule for 'survives'; staleness is the reviewer's next step, and refusing would make every sync on a not-yet-approved branch fail
  Rejected: a separate survives check; refusing on a stale approval
- 2026-10-22 [decision] merge --sync is one catch-up command under both merge strategies: git merge <base> into the owner branch (never a rebase), rerun, commit receipts, report approval, verdict; it integrates nothing
  Reason: Under fast_forward_only the base must come into the branch whenever it moved. Under merge_commit, atelier merge merges a clean branch itself (atelier-p189), so --sync matters for conflicts and merge_base_records, but its mechanics need no strategy branch; output ends in 'Next: atelier merge --check' either way
  Rejected: reading the base policy to vary behaviour per strategy; integrating after the sync
