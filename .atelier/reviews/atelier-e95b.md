---
schema: atelier.review
schema_version: 2
id: atelier-e95b
title: 'Integration and cleanup guidance outside atelier.md is still Git/PR-generic. Nothing explicitly says that in an Atelier repository you integrate only with ''atelier merge'' and never with plain git merge, which is the behavior the new ''Ungated merge'' eval row in maintainers/skill-evaluation.md expects. atelier.md describes merge but does not prescribe it. Also, branches-and-worktrees.md:39 defaults to naming branches agent/<issue>-<slug>, and its closing inventory (:94-95, ''git branch --list agent/*'') misses Atelier''s <issue_type>/<id> work branches, such as superseded work that merge never deleted. Suggested: one sentence in atelier.md making atelier merge the only integration path, and a note in the Closing inventory to include the Atelier branch pattern.'
target: atelier-kr30
identity: Independent reviewer (Claude subagent)
commit: 9b61daecaa3d48e3f1657d488d81fb32a1c0fc3a
created_at: 2026-10-15T19:58:25.648339816Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 2126d1366e05db6bfe584ff6d2bc448b59e12609
    recorded_at: 2026-10-15T19:59:39.049607595Z
    resolution_commit: 2126d1366e05db6bfe584ff6d2bc448b59e12609
    reason: Fixed in upstream f9f69af, resynced here
    migrated_from: atelier-46c0
---

Integration and cleanup guidance outside atelier.md is still Git/PR-generic. Nothing explicitly says that in an Atelier repository you integrate only with 'atelier merge' and never with plain git merge, which is the behavior the new 'Ungated merge' eval row in maintainers/skill-evaluation.md expects. atelier.md describes merge but does not prescribe it. Also, branches-and-worktrees.md:39 defaults to naming branches agent/<issue>-<slug>, and its closing inventory (:94-95, 'git branch --list agent/*') misses Atelier's <issue_type>/<id> work branches, such as superseded work that merge never deleted. Suggested: one sentence in atelier.md making atelier merge the only integration path, and a note in the Closing inventory to include the Atelier branch pattern.
