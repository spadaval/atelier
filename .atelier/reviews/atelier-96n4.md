---
schema: atelier.review
schema_version: 2
id: atelier-96n4
title: 'crates/atelier-cli/src/commands/git.rs:113-131 (unmerged_done_label after 0e3ffcd5): ''merged'' now requires the base branch''s own copy of .atelier/issues/<id>.md with the same status, so once prune removes that terminal record from the base (default retention 7 days), any work branch still kept locally (protected from branch cleanup, e.g. a checked-out or dirty worktree) is reported as ''[done on branch X, not merged into main]'' even though it was fast-forwarded into main. Reproduced in /tmp: close+commit on the branch, ff-merge into main, git rm the record on main and commit, then status from another branch and prime on the work branch both say ''not merged''. The wording errs on the safe side and needs a lingering branch, so it is low impact. Suggested fix (follow-up): when the base has no copy of the record, fall back to branch-tip ancestry only if the record was once present in the base history (e.g. git log on the path in refs/heads/<base>), which keeps the uncommitted-fresh-branch case correct.'
target: atelier-9k7o
identity: Independent reviewer (Claude subagent)
commit: ddc959416eb2392cda74049e62748de4bdb1108d
created_at: 2026-10-15T23:13:28.724791563Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 42d257ae8348117042f74992e3d553d465bef5b4
    recorded_at: 2026-10-15T23:13:58.992695798Z
    follow_up: atelier-lwdu
    reason: 'Accepted as recommended: only affects kept branches after prune and errs on the safe side; follow-up atelier-lwdu'
    migrated_from: atelier-pcch
---

crates/atelier-cli/src/commands/git.rs:113-131 (unmerged_done_label after 0e3ffcd5): 'merged' now requires the base branch's own copy of .atelier/issues/<id>.md with the same status, so once prune removes that terminal record from the base (default retention 7 days), any work branch still kept locally (protected from branch cleanup, e.g. a checked-out or dirty worktree) is reported as '[done on branch X, not merged into main]' even though it was fast-forwarded into main. Reproduced in /tmp: close+commit on the branch, ff-merge into main, git rm the record on main and commit, then status from another branch and prime on the work branch both say 'not merged'. The wording errs on the safe side and needs a lingering branch, so it is low impact. Suggested fix (follow-up): when the base has no copy of the record, fall back to branch-tip ancestry only if the record was once present in the base history (e.g. git log on the path in refs/heads/<base>), which keeps the uncommitted-fresh-branch case correct.
