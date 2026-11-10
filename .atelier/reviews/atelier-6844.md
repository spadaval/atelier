---
schema: atelier.review
schema_version: 2
id: atelier-6844
title: finding on atelier-syyj @90aa800a
target: atelier-syyj
identity: gpt-6.1-sol
commit: 90aa800ae182afac3260a14f6e264456d5bc14ef
created_at: 2026-11-10T17:59:50.447310037Z
event:
  event: finding
  recommendation: fix_now
---

high: next/crates/atelier-cli/src/merge.rs:47 Read the item and its approval facts from the captured head, not the mutable checkout. Capturing head before repo.current fixes the earlier source-commit race, but current still loads the item and reviews from the working tree through repo.rs:61-62. The subsequent diff is pinned to head while acts::stale compares Goal, Done when and Scope against those working-tree bytes. A clean status later does not prove those were the committed bytes. Reproduced offline at this head in a scratch repository with a local bare origin: approve the item, then commit a changed Goal. The control merge exits 1 and asks for submit. Temporarily put the old reviewed item bytes in the checkout, start merge, and restore the committed changed bytes after current reads them but before checkout runs git diff and status. A Git wrapper performs only that restoration on the first diff --name-only call. Merge then exits 0, advances main to a merge commit containing the unreviewed Goal, marks the item done, and leaves a clean checkout. The branch head never moves, so moved() cannot catch it. Read committed item/review facts against the exact captured hash for merge (while preserving the base done/dropped rule), and add a regression for a checkout changing during the initial reads; the new hold-after-read test changes a commit after the facts were gathered and does not cover this gap. Verdict: merge after this fix and fresh review, with no approval of this head. Sizes: complexity high (snapshots, temporary checkouts, locking and forge integration), value high (completes approved-to-done), risk high (silently merges requirements the approval does not cover). Contract and Fitness fail on this reproduction. All added moving parts otherwise trace to the stated merge, safety, forge and retry requirements. The prior status finding is fixed and its tests pass; the source-commit reproduction covered by the new regression also passes. Fresh proof: requested HEAD-parent and whole-range whitespace checks, root and next formatting, root and next offline locked metadata, next clippy --offline -j 4 --all-targets -- -D warnings, and nextest --offline --build-jobs 4 -j 4: 101 passed, none skipped. All tests used the requested TMPDIR and this worktrees next/target. No code under crates/ changed. Open product questions: none. Residual risk: real GitHub and network behavior were deliberately not exercised. Scratch repositories were removed; no code was edited.
