---
schema: atelier.review
schema_version: 2
id: atelier-mi8y
title: finding on atelier-295u @c42cee62
target: atelier-295u
identity: gpt-6-astra-amend2-r1
commit: c42cee62d694d5cb86961a2140d9f25ca52f8962
created_at: 2026-11-10T04:00:07.364067348Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: e63fa838a44b918b22ababb46442050d339e56df
    recorded_at: 2026-11-10T04:08:49.917665875Z
    follow_up: atelier-8b0a
    reason: The false claim is removed and the walk's requirement stated in e63fa838; how to make it one pass is deferred to the log step's planning, named in atelier-8b0a's Unknowns.
---

medium, docs/strategies/redesign-architecture.md:294 and atelier-8b0a: the claim that this is the single walk used by site history today is inaccurate. atelier-site/src/lib.rs:596 calls mission_review::debrief, whose timeline uses record_history::issue_histories; record_history.rs:45,125 explicitly uses first-parent history and collapses merged branch-local states into the merge. A separate full_histories walk exists for Done when edits at line 567. Reusing the debrief walk would lose start, submit, review rounds and take-overs after merge, including when the merged branch is deleted. Defer the traversal design to 8b0a full planning: use ancestry across all relevant parents, avoid counting imported changes twice, and require the plain and two-round histories to survive merge and branch deletion; correct the reuse claim then.
