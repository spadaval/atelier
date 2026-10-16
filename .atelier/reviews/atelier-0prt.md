---
schema: atelier.review
schema_version: 1
id: atelier-0prt
title: 'crates/atelier-workflow/src/lib.rs:152-158 and .atelier/workflow.yaml:142-148: the decide transition (to done) only runs decision.recorded; unlike work close it lacks blockers.none_open and work.terminal, so the decision gate is weaker than every other done transition. Reproduced: (a) `issue link <decision> <open task> --role blocked_by` then `issue transition <decision> decide` succeeds with the blocker open; (b) `issue create ''Child'' --parent <decision>` then decide succeeds with the child todo, after which the child is classified Backlog under a terminal ancestor and silently drops out of `work ready`. Suggested fix: add blockers.none_open and work.terminal to decide in the starter policy and this repo''s policy, and cover both in the test.'
target: atelier-ox63
identity: Independent reviewer (Claude subagent)
commit: f4376ada15eaa6ee51fc9b97dd78e228648811b5
created_at: 2026-10-16T01:12:08.769955871Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-workflow/src/lib.rs:152-158 and .atelier/workflow.yaml:142-148: the decide transition (to done) only runs decision.recorded; unlike work close it lacks blockers.none_open and work.terminal, so the decision gate is weaker than every other done transition. Reproduced: (a) `issue link <decision> <open task> --role blocked_by` then `issue transition <decision> decide` succeeds with the blocker open; (b) `issue create 'Child' --parent <decision>` then decide succeeds with the child todo, after which the child is classified Backlog under a terminal ancestor and silently drops out of `work ready`. Suggested fix: add blockers.none_open and work.terminal to decide in the starter policy and this repo's policy, and cover both in the test.
