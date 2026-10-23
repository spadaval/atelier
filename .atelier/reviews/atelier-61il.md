---
schema: atelier.review
schema_version: 1
id: atelier-61il
title: finding on atelier-0uaj @119161a9
target: atelier-0uaj
identity: Independent reviewer 4 (Claude Sonnet subagent)
commit: 119161a92069f25d4c62a7a86975428f34c91873
created_at: 2026-10-23T16:25:38.232151697Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-app/src/review_gate.rs:1030 (append_only) with crates/atelier-app/src/committed_snapshot.rs commit_command: the tip fix 'git reset --keep HEAD~1' is positional, but land lists it after a land_records_uncommitted fix that adds a commit. Scratch repro: approval deleted in a committed tip commit plus a new untracked issue record; land prints 1) 'git add .atelier && git commit -m …' 2) 'git reset --keep HEAD~1'. Running both in order makes the second drop the just-committed issue record (the deletion commit stays), silently losing the committed record (only in reflog); the next land then reprints reset. Suggest making the advice order-independent: print the commit-addressed 'git rebase --onto <commit>~1 <commit>' form even at the tip when other failing checks exist (it replays the later commit), or print the append-only fix first and say 'then rerun land' so the tip form is recomputed, and add a test that runs both printed fixes in order and asserts the record survives.
