---
schema: atelier.review
schema_version: 2
id: atelier-41xn
title: finding on atelier-l58v @9ea0a19b
target: atelier-l58v
identity: claude-code-reviewer (opus-5.5)
commit: 9ea0a19b4fb5a0f3488d5efd40e58bbaeff0d91b
created_at: 2026-11-08T21:49:01.859702317Z
event:
  event: finding
  recommendation: defer
---

medium: skills/agent-factory/references/atelier.md:15-16 vs :66-67, and procedures/orchestrate.md:89 (eb14a31). Declining a started item is now a normal ending (a stop signal's discard, a standing reject), but the skill does not say where its record goes or what happens to the branch: atelier.md says decline needs 'no branch, review or merge' while also saying 'a started item's own record changes only on its work branch'. A reader following the second sentence declines on the work branch, archives it, and the base shows the item in_progress forever. All declining eval runs guessed the same way (decline committed on main, branch tagged archive/<id> and deleted, its follow-ups and findings left only on the tag), so no failure was observed. Fix: one sentence in atelier.md saying a declined started item's record is committed on the base and its branch archive-tagged and deleted, and that follow-ups filed on that branch lapse with it.
