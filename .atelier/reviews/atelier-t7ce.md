---
schema: atelier.review
schema_version: 1
id: atelier-t7ce
title: 'medium: crates/atelier-cli/src/commands/record_commit.rs:66-71,132-134 A stale .atelier/runtime/staged-records.json makes --commit-records commit changes the command did not write, violating 4ixu''s constraint (never other staged or unstaged changes). begin() trusts every path in the pending file without checking it is still staged, and finish() re-runs ''git add --'' and ''git commit -- <paths>'' on all pending paths, which takes their working-tree content. Reproduced with target/debug/atelier in a scratch repo: ''evidence record --stage-records'', then a hand ''git commit'' (the documented hand-commit habit) leaves the pending file; an unstaged WIP edit to that earlier evidence file is then swept into the next ''evidence record --commit-records'' commit, whose subject also names the old record (''Record evidence <old> and 1 more''). A staged hand edit to a pending path is likewise accepted as ''not foreign''. Pending also survives branch switches in the worktree. Fix: in begin(), drop pending paths that have no staged change and refuse when a pending path has unstaged changes (git diff --name-only); in finish(), git add only the paths this command wrote; add a CLI test for stage, hand commit, then commit-records with a dirty earlier record.'
target: atelier-zuow
identity: Independent reviewer (Claude subagent)
commit: eb6ee5cf688f70c5521050bad0eac7d1d58da725
created_at: 2026-10-22T21:45:00.068397200Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/commands/record_commit.rs:66-71,132-134 A stale .atelier/runtime/staged-records.json makes --commit-records commit changes the command did not write, violating 4ixu's constraint (never other staged or unstaged changes). begin() trusts every path in the pending file without checking it is still staged, and finish() re-runs 'git add --' and 'git commit -- <paths>' on all pending paths, which takes their working-tree content. Reproduced with target/debug/atelier in a scratch repo: 'evidence record --stage-records', then a hand 'git commit' (the documented hand-commit habit) leaves the pending file; an unstaged WIP edit to that earlier evidence file is then swept into the next 'evidence record --commit-records' commit, whose subject also names the old record ('Record evidence <old> and 1 more'). A staged hand edit to a pending path is likewise accepted as 'not foreign'. Pending also survives branch switches in the worktree. Fix: in begin(), drop pending paths that have no staged change and refuse when a pending path has unstaged changes (git diff --name-only); in finish(), git add only the paths this command wrote; add a CLI test for stage, hand commit, then commit-records with a dirty earlier record.
