---
schema: atelier.review
schema_version: 1
id: atelier-a5vf
title: finding on atelier-r0fj @e5bf04ac
target: atelier-r0fj
identity: claude-review-r0fj
commit: e5bf04ac5f3ec715b8a5f76abbdad5d18e0a4bd0
created_at: 2026-11-05T00:22:46.189062285Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-app/src/record_history.rs:369-374 (written_at), interaction of the atelier-uev7 fix (79a5cf61) with atelier-b3qt. The walk now records the branch's entry commit E (the merge that brings in the fork) as one version that stands for fork..E, but written_at still dates a link by searching first_parent(E)..E. For a base merge that range holds only base commits, so a link written on the branch before it merged the base is not found, and the added item keeps E's commit time. Its addition then lands after its delivery, which is the ordering b3qt fixed. Repro (target/debug/atelier at e5bf04ac, fixture ~/.cache/atelier-tmp/r0fj-fixture2): start mission M on main with scope item P. On feature, create child C under P, close C, commit. Main changes another record; feature merges main. 'atelier debrief M' on feature prints 'delivered C' at 19:21, then 'added C' at 19:22. Before feature merged main, and on main after merging feature, C is added at 19:21, before its delivery. Fix: date the link from the range the version stands for (fork..E for the entry, first_parent..commit otherwise), e.g. carry that range or its base on RecordVersion/Moment. Add this case (a child added on the branch before it merges the base, debriefed from the branch) to a fixture.
