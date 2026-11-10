---
schema: atelier.review
schema_version: 2
id: atelier-gsim
title: finding on atelier-syyj @6b5dd8e6
target: atelier-syyj
identity: gpt-6.1-sol
commit: 6b5dd8e69230fb8a2571741c17919e872843784b
created_at: 2026-11-10T18:07:55.665791041Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/merge.rs:69 Keep the no-review-file condition of --trivial aware of untracked checkout reviews. Replacing branch.record with item_at(head) correctly pins requirements and approval facts, but also drops review files not committed yet. Only Refusal::Role from the earlier checkout-based check is retained, so its Reviewed condition is ignored. git::uncommitted deliberately excludes untracked files. Reproduced offline in a scratch repository with a local bare origin: start a building item, commit ticked Done when and a Report, add a valid reviews/1-zzzz.md without staging it, then run merge --trivial a-typo as the implementer. It exits 0 and advances main to a done item with the trivial reason, despite the existing review file. Also reproduced using a file created by the actual atelier review command: temporarily set the checkout item in_review, run review as another thread, and restore the committed building item; the tracked checkout is clean but --trivial still merges. Review deliberately leaves its file uncommitted until verdict, so this is a first-class record state. The Done when explicitly refuses --trivial on an item with a review file. Preserve the committed snapshot fix for requirements and approval, but check checkout review-file presence for the trivial path and add an untracked-review regression. This is R2, FIX NOW; no approval of the pinned head. The existing tracked-review refusal test passes but does not cover this case. Scratch repositories were removed; no code was edited.
