---
schema: atelier.review
schema_version: 1
id: atelier-k4pe
title: 'Reviewed f8387556 and the branch diff against master. Checked: (1) blockers.none_open skips superseded ids only; the owner''s own dependency_closure (which also includes its own blockers) is still collected, and non-superseded siblings keep their own and inherited blockers, so nothing the parent carries is hidden. (2) completion_claims blockers.transitive_none_open and evidence checks already skip superseded ids. (3) work.terminal counts superseded as done; the close/start validators (transitive_blockers_none_open, open_blockers in workflow_validation.rs) evaluate only the issue itself plus its parent''s inherited blockers, never a child''s, so they do not hold a parent to a superseded child; work ready classifies per issue. (4) The hierarchy is one level deep (create with a grandchild fails with workflow_issue_hierarchy_invalid), so no done grandchild can inherit a superseded child''s blockers. (5) The new test reproduces nr0w: it fails with the fix reverted and passes with it. (6) work-model.md wording is accurate. cargo nextest run --workspace: 629/629 pass in the worktree.'
target: atelier-lq42
identity: Independent reviewer (Claude subagent)
commit: 0c98c5383f71f05b86b61ed3e11214a62409b82c
created_at: 2026-10-16T19:09:45.487284817Z
event:
  event: approval
---

Reviewed f8387556 and the branch diff against master. Checked: (1) blockers.none_open skips superseded ids only; the owner's own dependency_closure (which also includes its own blockers) is still collected, and non-superseded siblings keep their own and inherited blockers, so nothing the parent carries is hidden. (2) completion_claims blockers.transitive_none_open and evidence checks already skip superseded ids. (3) work.terminal counts superseded as done; the close/start validators (transitive_blockers_none_open, open_blockers in workflow_validation.rs) evaluate only the issue itself plus its parent's inherited blockers, never a child's, so they do not hold a parent to a superseded child; work ready classifies per issue. (4) The hierarchy is one level deep (create with a grandchild fails with workflow_issue_hierarchy_invalid), so no done grandchild can inherit a superseded child's blockers. (5) The new test reproduces nr0w: it fails with the fix reverted and passes with it. (6) work-model.md wording is accurate. cargo nextest run --workspace: 629/629 pass in the worktree.
