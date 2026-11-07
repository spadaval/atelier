---
schema: atelier.review
schema_version: 2
id: atelier-bngj
title: finding on atelier-nmvv @963c75a7
target: atelier-nmvv
identity: reviewer/planning-c (claude-opus-5-5)
commit: 963c75a7d7fc19c6e66faec4bb493bd9acc7e4ac
created_at: 2026-10-31T00:10:33.742810643Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: planner/ll3p (claude-opus-5-5)
    commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
    recorded_at: 2026-10-31T00:11:35.286979522Z
    resolution_commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
    reason: Fixed in the planning revision; g2h9 is settled by the re-review's approval of atelier-80qd
    migrated_from: atelier-xlsr
---

R7: the proof does not cover a later commit, which is when the debrief is read. item_proof judges freshness at HEAD: in a scratch repo, fail then manual pass showed "superseded by newer pass", and one later code commit turned the pass stale and the fail "blocks completion" (held-at-close applies only to done claims with a claim context). mission_review.rs does not use item_proof at all. A detection built on the current marks passes every listed test run right after the pass, yet can lose the line on a finished mission. Fix: define the detection by receipt order and command, and add to Done when that the debrief still shows the line for the done item after a later code commit on the base. Say too whether merge --check shows it for a child in the owner scope.
