---
schema: atelier.review
schema_version: 1
id: atelier-tqgi
title: 'crates/atelier-app/src/review_gate.rs:944: the report-part exemption keys only on historical=true, which only claim_nodes sets for claims outside the owner''s scope. The owner''s scope (owner plus all descendants) goes through evaluate_session/evaluate (Session/Merge kind, historical=false), so a child closed on a parent''s branch before the policy required the report fails land and merge once the policy reaches that branch. Reproduced in a scratch repo: parent P started on task/P, child C closed with --reason and committed, base then requires report.*, branch merges base, atelier land fails with required_field.report.{deviations,decisions,surprises,open_items} on C. This breaks the Done-when ''Done records committed before the policy required a part stay valid'', and work-model.md''s ''land checks required parts on new done claims only'' overstates it. No in-flight parent in this repository is affected today. Fix: decide part requirements per claim from the policy in force when the claim was committed (the issue''s Unknowns names this route), or at least exempt scope claims whose done status predates the policy change; add a session/merge test with a scoped child.'
target: atelier-sfsq
identity: Independent reviewer (Claude subagent)
commit: 184469851584ee81ba2b163fb54071bd9ccabd7b
created_at: 2026-10-17T18:04:29.888216116Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/review_gate.rs:944: the report-part exemption keys only on historical=true, which only claim_nodes sets for claims outside the owner's scope. The owner's scope (owner plus all descendants) goes through evaluate_session/evaluate (Session/Merge kind, historical=false), so a child closed on a parent's branch before the policy required the report fails land and merge once the policy reaches that branch. Reproduced in a scratch repo: parent P started on task/P, child C closed with --reason and committed, base then requires report.*, branch merges base, atelier land fails with required_field.report.{deviations,decisions,surprises,open_items} on C. This breaks the Done-when 'Done records committed before the policy required a part stay valid', and work-model.md's 'land checks required parts on new done claims only' overstates it. No in-flight parent in this repository is affected today. Fix: decide part requirements per claim from the policy in force when the claim was committed (the issue's Unknowns names this route), or at least exempt scope claims whose done status predates the policy change; add a session/merge test with a scoped child.
