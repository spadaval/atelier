---
schema: atelier.review
schema_version: 1
id: atelier-ca6u
title: 'MEDIUM crates/atelier-cli/src/commands/merge.rs merge_unproven guard and merge.merged_tree node; workflow_validation.rs record builtins: the guard accepts any non-empty merge_checks, but only shell.check (and git.worktree_clean) look at the merged checkout; lint.none_blocking, evidence.attached, blockers.*, issue.sections_parseable evaluate the branch-head index. Probe: merge_checks: [lint.none_blocking] passes the guard and lands a merge commit while the gate prints ''merge_checks ran on merged tree'' and the commit message says the checks passed on this tree, though nothing examined it. This defeats the ''merged tree''s proof is explicit'' requirement. The node is also unconditionally passed: true. Fix: require at least one tree-executing check (shell.check) under merge_commit and name only those as the merged-tree proof, or make the node fail when none ran.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: 7185c3d00f161519ba59396b48f26ff2595cd28b
created_at: 2026-10-17T06:10:47.805401948Z
event:
  event: finding
  recommendation: fix_now
---

MEDIUM crates/atelier-cli/src/commands/merge.rs merge_unproven guard and merge.merged_tree node; workflow_validation.rs record builtins: the guard accepts any non-empty merge_checks, but only shell.check (and git.worktree_clean) look at the merged checkout; lint.none_blocking, evidence.attached, blockers.*, issue.sections_parseable evaluate the branch-head index. Probe: merge_checks: [lint.none_blocking] passes the guard and lands a merge commit while the gate prints 'merge_checks ran on merged tree' and the commit message says the checks passed on this tree, though nothing examined it. This defeats the 'merged tree's proof is explicit' requirement. The node is also unconditionally passed: true. Fix: require at least one tree-executing check (shell.check) under merge_commit and name only those as the merged-tree proof, or make the node fail when none ran.
