---
schema: atelier.review
schema_version: 1
id: atelier-ss2x
title: 'low: crates/atelier-cli/tests/session.rs:3248 the new test covers close_only, children_close_only, with_code and with_goal, all single-claim and all fast-forward. The behaviour the commit calls out as a surprise (base_done_claims returning every claim so a covered claim cannot hide an uncovered one), the owner-plus-outside-issue multi-claim commit, a sibling or parent close after approval, a note smuggled into the close, and a merge_commit-strategy integration are untested. I exercised the multi-claim, sibling and note cases in a scratch copy and they behave correctly (land refuses, merge refuses except the new-done-issue case in the fix_now finding), so this is regression-protection only. Add the multi-claim and sibling cases.'
target: atelier-1ztv
identity: Independent reviewer (Claude subagent)
commit: f7e1cefa82d568b23f9b06cfc193ab350def07c1
created_at: 2026-10-22T21:12:53.254979087Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/tests/session.rs:3248 the new test covers close_only, children_close_only, with_code and with_goal, all single-claim and all fast-forward. The behaviour the commit calls out as a surprise (base_done_claims returning every claim so a covered claim cannot hide an uncovered one), the owner-plus-outside-issue multi-claim commit, a sibling or parent close after approval, a note smuggled into the close, and a merge_commit-strategy integration are untested. I exercised the multi-claim, sibling and note cases in a scratch copy and they behave correctly (land refuses, merge refuses except the new-done-issue case in the fix_now finding), so this is regression-protection only. Add the multi-claim and sibling cases.
