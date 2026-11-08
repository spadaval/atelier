---
schema: atelier.review
schema_version: 2
id: atelier-v9fx
title: finding on atelier-stnv @f684caa1
target: atelier-stnv
identity: grok-plan-reviewer (grok-4.7)
commit: f684caa15b2bb4fe2a68878e52a27c3499a1d9df
created_at: 2026-11-08T05:49:42.804733961Z
event:
  event: finding
  recommendation: fix_now
---

high: The won't-fix identity rule contradicts itself and current Atelier. The first Done when line says won't-fix uses no_action's identity rules and that an author cannot decide findings on its own work, and the tests line says an author's won't-fix is refused. docs/product/reviews-and-merge.md (Dispositions) says any identity may record a defer or no_action and that the identity is not compared with the work's Git authors. review disposition stores that identity and only rejects an empty one (crates/atelier-cli/src/commands/review.rs disposition; crates/atelier-records/src/review.rs validate_disposition). An implementer cannot both match no_action and refuse the author, and there is no author field to compare. Keep one rule: match no_action and drop the refusal and the test, leaving the author ban to the skill, or drop the 'same identity rules' clause, define the new check, and test that.
