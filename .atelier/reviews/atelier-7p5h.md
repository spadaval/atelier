---
schema: atelier.review
schema_version: 2
id: atelier-7p5h
title: finding on atelier-syyj @4cb9d2c2
target: atelier-syyj
identity: gpt-6.1-sol
commit: 4cb9d2c2b0b6cae6b183e4f423244840d503eb66
created_at: 2026-11-10T18:13:33.081423936Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/commands.rs:24 Use the committed item for the stale-approval facts shown by status, as merge does. status passes the mutable checkout record to Repo::checkout and entry, whereas merge replaces it with item_at(head) before judging approval. Reproduced offline on this head using the same setup as merge_judges_the_committed_item_not_the_checkouts: approve round 1, commit a Goal change from Print hello to Print hello twice, then put the reviewed Goal back in the checkout without committing. status omits the stale approval and tells the caller to commit or stash and then run atelier merge; merge names this item Goal as changed since round 1 and says Then: atelier submit. Thus the required status/merge agreement still fails in the very scenario the round 2 fix added. Keep role and draft-review visibility from the checkout, but derive merge staleness and the resulting next command from the captured committed item in both commands. Extend that regression to assert status names the same change and submit. R2, FIX NOW; no approval of the pinned head. The scratch repository was removed; no code was changed.
