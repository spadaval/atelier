---
schema: atelier.review
schema_version: 2
id: atelier-ttao
title: finding on atelier-syyj @4cb9d2c2
target: atelier-syyj
identity: gpt-6.1-sol
commit: 4cb9d2c2b0b6cae6b183e4f423244840d503eb66
created_at: 2026-11-10T18:13:23.774771697Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/git.rs:291 Count ignored review files when refusing merge --trivial. The new untracked-file guard runs ls-files --others --exclude-standard, so a review ignored by .git/info/exclude or a global ignore is absent from both the committed ItemDir and the uncommitted list. Reproduced offline on this head: start a building item with ticked Done when and a Report, ignore its reviews/ directory in .git/info/exclude, temporarily set its checkout status to in_review, run atelier review as r1 (which successfully creates a real review file), then restore the committed building item. git status is clean, but merge --trivial a-typo exits 0 and advances main to done despite that review file. This violates the explicit no-review-file condition, independently of Git ignore preferences. Check checkout review-file presence for the trivial path regardless of ignore rules (and again before writing), or include ignored files in the item-directory guard. Add an ignored-review regression alongside the untracked-review test. R1, FIX NOW; no approval of the pinned head. The scratch repository was removed; no code was changed.
