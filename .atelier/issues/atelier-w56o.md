---
created_at: "2026-10-17T01:36:20.763168413+00:00"
id: "atelier-w56o"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-w56o
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
    id: "atelier-tciw"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Tell orchestrators to update a waiting branch only when it is next to land"
updated_at: "2026-10-17T05:28:32.716906908+00:00"
---

## Goal

The Agent Factory orchestrate guidance let the astromech orchestrator bring main into every waiting branch after each landing: "Regenerating lockfiles, committing each merge, and running the full checks on all four branches" (2026-10-16). feature/astromech-cmhv absorbed main 16 times over 4.5 hours. Eight of those merges needed a review of their conflict resolution. One catch-up merge, made when cmhv was next to land, would have been enough. The shell's separate `app/src-tauri/Cargo.lock` was also first found broken by a reviewer (a `fix_now` finding: missing base64 and flate2), not by a check.

## Done when

The vendored `.agents/skills/agent-factory` orchestrate procedure, and the upstream skill in spadaval/.agents, say:

- Review a branch when its work is done, and leave waiting branches alone when the base moves.
- When a branch is next to land:
  - merge the base into it once;
  - regenerate generated files such as lockfiles in that step;
  - rerun proof;
  - get the merge reviewed if it needed resolving or regenerating anything;
  - then run `atelier merge`.
- Put generated-file consistency checks in required proof or `merge_checks`, so a merged but broken lock is caught mechanically. For a Rust workspace that means `cargo metadata --locked` for every lockfile.

## Context

This is a guidance-only change. Together with the merge-readiness view it removes most catch-up churn without changing the merge model. From the 2026-10-16 transcript retrospective.

## Notes

- 2026-10-17 [note] 2026-10-17: dropped the done-when line 'Land branches that touch the same files back to back' after review finding R2: once each branch merges the base only when it is next to land, landing order does not change how many conflicted merges or resolution reviews there are (for branches sharing a file, all but the first conflict in any order). Merges that regenerate generated files now also need review (R1). Mission S2 is unaffected.
