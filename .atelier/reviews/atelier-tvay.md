---
schema: atelier.review
schema_version: 1
id: atelier-tvay
title: 'Re-reviewed through 4d20c51c and bc46a9a1. atelier-17it fixed: code committed on the base after a local install and before pushing is refused with land_base_unreviewed (scratch /tmp/rv40i6/c). The fork case is closed because only the first-parent install commit and what it merges in are exempt. atelier-594w fixed: land''s install guidance and workflow-configuration.md now say to land as one merge commit (merge the PR on the host and pull, or git merge --no-ff) and that a fast-forward is refused. Confirmed in /tmp/rv40i6/d: fast-forward refused, --no-ff lands, and the guidance text is shown on the install branch. The test covers the fast-forward refusal, the --no-ff landing, code after install, the shortened path list, and gates after push. The branch binary in astromech-atelier chore/atelier-tracker gives exit 0 with the new guidance (read-only, worktree unchanged). Earlier checks still hold: stale local base and policy only on a remote copy stay gated on unowned branches. Docs match the code. atelier-jsd4 is deferred to atelier-bibd. Re-adding the policy after removing it on the base is a gap already on master that this change does not worsen. Evidence atelier-0obx is a passing workspace nextest at 4d20c51c.'
target: atelier-40i6
identity: Independent reviewer (Claude subagent)
commit: bc46a9a1ed7668cde32627a5d80364d62ffce8ed
created_at: 2026-10-16T00:06:39.504768130Z
event:
  event: approval
---

Re-reviewed through 4d20c51c and bc46a9a1. atelier-17it fixed: code committed on the base after a local install and before pushing is refused with land_base_unreviewed (scratch /tmp/rv40i6/c). The fork case is closed because only the first-parent install commit and what it merges in are exempt. atelier-594w fixed: land's install guidance and workflow-configuration.md now say to land as one merge commit (merge the PR on the host and pull, or git merge --no-ff) and that a fast-forward is refused. Confirmed in /tmp/rv40i6/d: fast-forward refused, --no-ff lands, and the guidance text is shown on the install branch. The test covers the fast-forward refusal, the --no-ff landing, code after install, the shortened path list, and gates after push. The branch binary in astromech-atelier chore/atelier-tracker gives exit 0 with the new guidance (read-only, worktree unchanged). Earlier checks still hold: stale local base and policy only on a remote copy stay gated on unowned branches. Docs match the code. atelier-jsd4 is deferred to atelier-bibd. Re-adding the policy after removing it on the base is a gap already on master that this change does not worsen. Evidence atelier-0obx is a passing workspace nextest at 4d20c51c.
