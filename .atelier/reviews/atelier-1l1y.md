---
schema: atelier.review
schema_version: 2
id: atelier-1l1y
title: finding on atelier-fcb7 @378eb60a
target: atelier-fcb7
identity: grok-4.6-plan-r2
commit: 378eb60a7695bdea855a47332bda78b1218b3445
created_at: 2026-11-10T01:23:29.137631872Z
event:
  event: finding
  recommendation: fix_now
---

medium docs/strategies/redesign-architecture.md:221-234: a started item is a local branch named for its ID, but work on the other host is visible after a fetch, as a remote-tracking branch, and is read the same way. start does not fetch or inspect refs/remotes/*/<id>, and Git's atomic create applies only inside one .git. The two Linux hosts can both start the same ready item. Smallest fix: start fetches and treats refs/heads/<id> and refs/remotes/*/<id> as already started; engh's two-worker test adds a fetch-from-another-clone case.
