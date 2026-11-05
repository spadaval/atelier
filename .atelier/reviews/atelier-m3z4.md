---
schema: atelier.review
schema_version: 1
id: atelier-m3z4
title: approval on atelier-iozu @0b3b4817
target: atelier-iozu
identity: claude-review-iozu
commit: 0b3b481729fb60b4d24e8b499c677767760a40bd
created_at: 2026-11-05T01:40:13.893281449Z
event:
  event: approval
---

Reviewed 8e7a8027..0b3b4817 against ypks, 4vda, pth2 Done when. ypks: no-flag close prints the kept Outcome; --reason keeps parts only when the transition requires a report part, so supersede drops them; the deviation (no transition can leave done, confirmed at atelier-workflow lib.rs:1536) is sound. 4vda: held claims now judged by the base policy at the claim's introducing commit rather than skipped; the deviation from 'policy at the parent' is justified (it would refuse pre-policy claims); work-model.md and reviews-and-merge.md state the rule and the side-branch limit as the orchestrator directed; new session test would fail on the old skip. pth2: helper gone, comments updated, workspace builds with no warnings. Ran targeted atelier-cli report/claim tests (66 pass), atelier-app tests (107 pass), fmt, metadata --locked, diff --check; doctor with the new binary on this repo still holds all 235 done claims (114 without reports). CI test jobs pass on ubuntu and macos for PR #24 at this head. No findings.
