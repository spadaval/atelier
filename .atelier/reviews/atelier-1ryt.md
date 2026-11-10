---
schema: atelier.review
schema_version: 2
id: atelier-1ryt
title: finding on atelier-fcb7 @36de8f1a
target: atelier-fcb7
identity: grok-4.6-amend2-r1
commit: 36de8f1abc4b2ec76b1eb30ca8b9f85588d82929
created_at: 2026-11-10T04:06:30.566658382Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: a3a04c3a6525f6bdb5ea0b5062b73c777edf0f89
    recorded_at: 2026-11-10T04:08:50.347151530Z
    follow_up: atelier-8b0a
    reason: The false claim is removed and the walk's requirement stated in e63fa838; how to make it one pass is deferred to the log step's planning, named in atelier-8b0a's Unknowns.
---

medium, redesign-architecture.md:286-295: The log says it is the one walk the site's history views make today. Today's walk in crates/atelier-app/src/record_history.rs is git log --first-parent --diff-merges=first-parent on HEAD and drops branch-local states; the site enumerates local branches only for current tips (crates/atelier-site/src/lib.rs work in flight). Consequence: if 8b0a reuses that walk, start, submit, review, verdict and take-over never appear, because start's building commit is not on the base (atelier-engh) and the merge's first parent jumps ready to done. Defer to 8b0a planning: drop that clause and walk the base plus each started-item branch and merge second parents.
