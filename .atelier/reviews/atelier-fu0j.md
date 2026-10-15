---
schema: atelier.review
schema_version: 1
id: atelier-fu0j
title: 'crates/atelier-cli/src/commands/start.rs:173-176 now snapshots refs/heads/<base> on every start with git::Snapshot::at(base) and propagates its error. If the local base does not yet contain the tracker, start now refuses where master succeeds. That is the case when Atelier is being adopted on a branch (installed and reviewed on a work branch before it reaches the base). Reproduced: main has code only and an upstream. git switch -c adopt; atelier init; create an issue; commit; atelier start <id>. This change gives exit 1: ''branch_snapshot: cannot read tracker at 46cf4ae6...: fatal: pathspec ''.atelier'' did not match any files'' plus tar noise. The master binary (target/debug/atelier in the main checkout) starts the same issue fine (Base: adopt). The snapshot also runs before check_base_commits looks for an upstream, so repos with no upstream pay for an archive extraction they never use. Suggested fix: skip the base check when the base commit has no workflow policy or tracker (git ls-tree <base> -- .atelier/workflow.yaml is empty), consistent with the bootstrap exemption. Resolve <base>@{upstream} before taking the snapshot. Add a test that starts work on an adoption branch whose base has no .atelier.'
target: atelier-3mjv
identity: Independent reviewer (Claude subagent)
commit: c27cba0bbd95f5a60880653317f3433b041ed1c5
created_at: 2026-10-15T23:42:45.296184678Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/start.rs:173-176 now snapshots refs/heads/<base> on every start with git::Snapshot::at(base) and propagates its error. If the local base does not yet contain the tracker, start now refuses where master succeeds. That is the case when Atelier is being adopted on a branch (installed and reviewed on a work branch before it reaches the base). Reproduced: main has code only and an upstream. git switch -c adopt; atelier init; create an issue; commit; atelier start <id>. This change gives exit 1: 'branch_snapshot: cannot read tracker at 46cf4ae6...: fatal: pathspec '.atelier' did not match any files' plus tar noise. The master binary (target/debug/atelier in the main checkout) starts the same issue fine (Base: adopt). The snapshot also runs before check_base_commits looks for an upstream, so repos with no upstream pay for an archive extraction they never use. Suggested fix: skip the base check when the base commit has no workflow policy or tracker (git ls-tree <base> -- .atelier/workflow.yaml is empty), consistent with the bootstrap exemption. Resolve <base>@{upstream} before taking the snapshot. Add a test that starts work on an adoption branch whose base has no .atelier.
