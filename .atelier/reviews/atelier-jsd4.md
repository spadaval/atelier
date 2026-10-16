---
schema: atelier.review
schema_version: 1
id: atelier-jsd4
title: 'crates/atelier-cli/src/commands/session.rs:246-261 (tracker_changes_since): an unowned branch is treated as installing Atelier whenever every copy of the base named in its own working-tree policy lacks the policy file. The branch controls that name. Reproduced in /tmp/rv40i6/b: origin/main has Atelier, feature/x creates an issue record and changes branch_policy.base_branch to old-release, an existing branch from before adoption. The branch binary''s land passes with ''Installing Atelier: old-release has no workflow policy yet''. The master binary refuses with land_work_untraceable. This needs a deliberate policy edit, and atelier merge still requires an owner, so the Stop-hook check is weakened but no code reaches the base. Suggested fix: count the branch as an install only if the policy at HEAD comes from commits unique to it, e.g. no commit in the history of .atelier/workflow.yaml is reachable from another local or remote-tracking branch. Or refuse when HEAD''s policy names a base other than the one in the base copies'' history.'
target: atelier-40i6
identity: Independent reviewer (Claude subagent)
commit: 1e086632e4d1bb87558b7a9ecc3fa5128e7a7839
created_at: 2026-10-15T23:59:50.438581035Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/session.rs:246-261 (tracker_changes_since): an unowned branch is treated as installing Atelier whenever every copy of the base named in its own working-tree policy lacks the policy file. The branch controls that name. Reproduced in /tmp/rv40i6/b: origin/main has Atelier, feature/x creates an issue record and changes branch_policy.base_branch to old-release, an existing branch from before adoption. The branch binary's land passes with 'Installing Atelier: old-release has no workflow policy yet'. The master binary refuses with land_work_untraceable. This needs a deliberate policy edit, and atelier merge still requires an owner, so the Stop-hook check is weakened but no code reaches the base. Suggested fix: count the branch as an install only if the policy at HEAD comes from commits unique to it, e.g. no commit in the history of .atelier/workflow.yaml is reachable from another local or remote-tracking branch. Or refuse when HEAD's policy names a base other than the one in the base copies' history.
