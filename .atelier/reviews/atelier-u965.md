---
schema: atelier.review
schema_version: 2
id: atelier-u965
title: finding on atelier-stnv @f684caa1
target: atelier-stnv
identity: grok-plan-reviewer (grok-4.7)
commit: f684caa15b2bb4fe2a68878e52a27c3499a1d9df
created_at: 2026-11-08T05:49:43.151493695Z
event:
  event: finding
  recommendation: fix_now
---

high: declined would not be treated as superseded, so it cannot finish work on the base. Done when adds a category-done declined status from backlog, todo and in_progress, and says work views, debrief and doctor show it like superseded. Land allows supersede on the base and refuses every other new done claim as land_base_done_claim (docs/product/reviews-and-merge.md, Planning changes on the base). The skip is the status string superseded, in crates/atelier-cli/src/commands/git.rs base_done_claims. Merge's completion claims apply blocker checks to every terminal status except that string (crates/atelier-app/src/review_gate.rs completion_claims). Loose ends and validations are work items but use the on_base workflow, which has no in_progress (crates/atelier-workflow/src/lib.rs starter policy and .atelier/workflow.yaml), so one from-list does not cover them. A decline of a backlog task on master would be refused as an unreviewed done claim, and a decline of a blocked item would fail the completion gate supersede skips. Name declined beside superseded on those paths, test that land accepts a base commit that only declines an item and that an open blocker does not refuse it, and give on_base a decline from backlog and todo.
