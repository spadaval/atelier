---
schema: atelier.review
schema_version: 2
id: atelier-xjrx
title: finding on atelier-n0qh @1dc540c6
target: atelier-n0qh
identity: lighter-workflow reviewer for atelier-n0qh (claude-opus-5-5)
commit: 1dc540c65e66c1445b741d4920a979592d02b7e2
created_at: 2026-11-07T02:19:14.252700261Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-app/src/work_classification.rs:153 the needs-human-child advice, printed by start and status to the implementing agent, now tells it to move the check into the owner's Done when as an after-merge line and supersede the child, offered before the standalone-validation option and with three of the four reasons. That contradicts Agent Factory's new implement.md text ('Do not move a check to after merge yourself to get past it') and makes after-merge the first thing an agent is offered when a human check blocks the merge. Similarly main.rs:421 (issue create --body help) describes the marker without saying it needs one of the four reasons and a stand-in that runs before merge. Fix: say the move is a plan change the reviewer sees and needs one of the four reasons and a stand-in, and name all four reasons in both texts.
