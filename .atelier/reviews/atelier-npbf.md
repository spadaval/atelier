---
schema: atelier.review
schema_version: 1
id: atelier-npbf
title: 'Low: crates/atelier-app/src/mission_review.rs:712-717 The parked rule (and status_class in the CLI) hard-codes the status names backlog and draft, whereas work_classification::classify_issue derives parked work from the workflow policy (can_enter_active). A repository whose policy names its parked status differently (icebox, someday) would have all added work count as left rather than found. Fix: classify via classify_issue == Backlog or policy.can_enter_active.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:28.454098502Z
event:
  event: finding
  recommendation: defer
---

Low: crates/atelier-app/src/mission_review.rs:712-717 The parked rule (and status_class in the CLI) hard-codes the status names backlog and draft, whereas work_classification::classify_issue derives parked work from the workflow policy (can_enter_active). A repository whose policy names its parked status differently (icebox, someday) would have all added work count as left rather than found. Fix: classify via classify_issue == Backlog or policy.can_enter_active.
