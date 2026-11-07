---
schema: atelier.review
schema_version: 2
id: atelier-epcv
title: finding on atelier-l457 @bc807963
target: atelier-l457
identity: independent reviewer subagent (claude-opus-5-5), atelier-l457
commit: bc8079639a940804bbdf8528a72e098c84f45e62
created_at: 2026-11-07T11:17:38.222732232Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 2a4ab52b50eed6252f5f2999eacdd21bff59a148
    recorded_at: 2026-11-07T11:28:16.625721060Z
    resolution_commit: 2a4ab52b50eed6252f5f2999eacdd21bff59a148
    reason: 'Fixed here instead of deferring: start ignores untracked files outside .atelier/, the same cut as this item''s untracked-files line.'
---

low: crates/atelier-cli/src/commands/start.rs:88 still refuses untracked files (start_checkout_dirty, through git_freshness::working_paths), so the files merge now leaves in place, and says it leaves, block the next `atelier start`. Reproduced in a scratch repo: `atelier merge` passed with `notes` and `scratch/deep/a` untracked and kept them, then `atelier start <next>` on the base refused naming both, advising "Commit or stash them", but plain `git stash` leaves untracked files. The Report lists this under Open items, but no follow-up issue exists in .atelier/issues, so it can be lost. Start is outside this item's Scope and refused untracked files before it, so no regression: file a follow-up, linked to atelier-mmpv, for start to ignore untracked files that branching at the current commit cannot overwrite, as merge and land now do.
