---
schema: atelier.review
schema_version: 1
id: atelier-e1ew
title: 'crates/atelier-app/src/decisions.rs:41: RecordDecision::is_current returns true for a decision record that is not decided yet (todo/blocked), although decided() (line 141) and the issue''s own decision say an open proposal is not in force. Failure: in a mission with an open decision record, `work decisions` lists it under ''Decision Records'' next to decided ones, and `work decisions -q` (documented in human-cli-output.md as ''the IDs of the items holding current decisions'') prints it; reproduced in a scratch repo (an open record fip8-review-fym4 printed by -q). S3 requires views to show which decisions are current, and atelier-1mk4/S6 will reuse is_current. Fix: make is_current require decided(policy, issue) as well as no superseded_by; show undecided records in their own section (e.g. ''Open Decision Records'') and leave them out of -q; add a test with an open decision record in scope.'
target: atelier-fip8
identity: Independent reviewer (Claude subagent)
commit: cc8bd55a9665bf7504da8bdb128b240bab442e8a
created_at: 2026-10-17T17:43:39.184026012Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/decisions.rs:41: RecordDecision::is_current returns true for a decision record that is not decided yet (todo/blocked), although decided() (line 141) and the issue's own decision say an open proposal is not in force. Failure: in a mission with an open decision record, `work decisions` lists it under 'Decision Records' next to decided ones, and `work decisions -q` (documented in human-cli-output.md as 'the IDs of the items holding current decisions') prints it; reproduced in a scratch repo (an open record fip8-review-fym4 printed by -q). S3 requires views to show which decisions are current, and atelier-1mk4/S6 will reuse is_current. Fix: make is_current require decided(policy, issue) as well as no superseded_by; show undecided records in their own section (e.g. 'Open Decision Records') and leave them out of -q; add a test with an open decision record in scope.
