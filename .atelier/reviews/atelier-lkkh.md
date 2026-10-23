---
schema: atelier.review
schema_version: 1
id: atelier-lkkh
title: finding on atelier-0uaj @93e82219
target: atelier-0uaj
identity: Independent reviewer 5 (Claude Sonnet subagent)
commit: 93e822196a3baede9b848a94d926edbdd224e6be
created_at: 2026-10-23T17:02:21.057775972Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/git.rs operation_in_progress checks rebase-merge, rebase-apply and MERGE_HEAD only, not CHERRY_PICK_HEAD or REVERT_HEAD. With a conflicted cherry-pick or revert plus a committed review deletion, land prints review_append_only with 'git rebase --autostash --onto <sha>~1 <sha>', which fails ('Cannot save the current index state / Cannot autostash') and the cause is not named. Not a worse state (nothing changes), gate outcome unchanged. Suggested: also detect CHERRY_PICK_HEAD/REVERT_HEAD and print 'git cherry-pick --abort' / 'git revert --abort'.
