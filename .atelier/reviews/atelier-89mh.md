---
schema: atelier.review
schema_version: 2
id: atelier-89mh
title: approval on atelier-jnrh @f97ee15e
target: atelier-jnrh
identity: independent reviewer subagent (claude-opus-5-5), atelier-jnrh
commit: f97ee15ecf1c8afb2c5467465de943527c9fa9f1
created_at: 2026-11-07T08:18:12.363535986Z
event:
  event: approval
---

Re-review of the base-merge resolution f97ee15e (parents 30663224 and a3c953cd, which brings in atelier-ilba), read with git show --remerge-diff. What it changes beyond Git's own merge: merge.rs runs record_guard::check_merge on the merge commit itself, so the done marks are checked; ReviewBase drops its fast_forward case, so review records can be corrected until they reach the base under every policy; committed_snapshot's comment; PRODUCT_INTENT.md, ADR 0028 and reviews-and-merge.md describe that; and four tests in merge_gates.rs and session.rs now expect correction, where they used to expect the old fast-forward finality. They still test the right things: a finding on the base stays append-only (review_rewrites... now puts it on main first), a finding not yet on the base can be corrected, and an approval edited before the base is still refused with review_append_only. Exercised in a scratch clone of astromech (fast_forward_only, old work close): an open child without a Report got the merge note and required_field.close_reason, not a record_invalid error; after it was ticked and reported, merge put both items on main as done through check_merge and land passed; editing the approval on the branch was refused with review_append_only. doctor, which checks the whole review history under the new ReviewBase, passes on clones of astromech, demiurge and life-on-rails; cosplay-studio's doctor fails with "review needs YAML front matter", as the installed binary's does. Proof at f97ee15e: git diff --check, cargo fmt --check and cargo metadata --locked are clean; cargo nextest run -j 4 --workspace passed 1001, and with --features test-hooks passed 1007; target/debug/atelier check passes. CI on PR 37 had passed on Linux, and macOS was still running when this was recorded.
