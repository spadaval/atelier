---
schema: atelier.review
schema_version: 2
id: atelier-pfbp
title: finding on atelier-hwxm @85722ed7
target: atelier-hwxm
identity: lighter-workflow reviewer for atelier-hwxm (claude-opus-5-5)
commit: 85722ed7497d8ed7038a2b572684f4c36c5c4cb8
created_at: 2026-11-07T01:19:25.740136486Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-app/src/review_gate.rs:348 and :1941 On the new read path, separate disposition events can be deleted or re-added freely, and the read disposition is judged at the event's first addition, so an unmigrated repository loses refusals the old binary has and is weaker than the migrated form (contradicts 'same results as after migration' and the non-author rule). Reproduced on fresh fixtures, with the installed binary writing the v1 events and this branch's binary judging: (1) Reviewer no_action E1 then fix_now E2 (no resolution), item done: both binaries refuse findings.disposed; git rm E2 and commit: the old binary refuses review_append_only, the new one passes doctor with E1's no_action. (2) Helper no_action event E1, Helper then authors a commit, Other runs review disposition (field written, E1 removed), then git checkout HEAD~1 -- .atelier/reviews/ restores the v1 finding and E1: doctor passes with Helper's no_action, judged at E1's original commit; the migrated-form equivalent (atelier-w0m7's case) is now refused. This is the current path for astromech, demiurge and cosplay-studio. Fix: in review_change_allowed accept removing a retired event only in a commit whose finding file carries its own disposition field and leaves none of that finding's events (what review disposition and migrate write), and refuse adding a retired event whose path existed earlier in history (or judge a read-path disposition at the latest commit that added its event / changed the finding's event set); add both cases to session.rs. R2/atelier-eial's 'narrow, pre-upgrade only' reason no longer holds with the read path.
