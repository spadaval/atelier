---
schema: atelier.review
schema_version: 1
id: atelier-320e
title: 'crates/atelier-cli/src/commands/session.rs:213-216 (tracker_changes_since): the fork point is the merge base with git::work_ref(base), which prefers the local refs/heads/<base> even when it lags its remote-tracking ref. A PR branch checked out from origin/main (gh pr checkout, git switch -c pr origin/main), or a feature branch that merged origin/main, while local main is stale, gets a merge base below the upstream record commits, so records other people landed on origin/main show up as changes of this branch. Reproduced in /tmp: main with one commit, origin/main one record commit ahead, git switch -c pr origin/main, code-only commit, atelier land exits 1 with land_work_untraceable naming .atelier/issues/demo-7nnz.md, a record the branch never touched. That brings back the exact every-stop hook error the record is meant to remove, for a common setup (the user''s own notes say local master here often lags origin). Suggested fix: compute changes against every candidate base ref (local head plus remote-tracking refs with the same name) and report only paths changed relative to all of them, or use the newest merge base among those refs; add a test with a stale local base and a newer remote-tracking base.'
target: atelier-fxbg
identity: Independent reviewer (Claude subagent)
commit: e4e1b6e78c9291ecbd3923b465f2cfd7f2e4a7a9
created_at: 2026-10-15T22:59:09.088874157Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/session.rs:213-216 (tracker_changes_since): the fork point is the merge base with git::work_ref(base), which prefers the local refs/heads/<base> even when it lags its remote-tracking ref. A PR branch checked out from origin/main (gh pr checkout, git switch -c pr origin/main), or a feature branch that merged origin/main, while local main is stale, gets a merge base below the upstream record commits, so records other people landed on origin/main show up as changes of this branch. Reproduced in /tmp: main with one commit, origin/main one record commit ahead, git switch -c pr origin/main, code-only commit, atelier land exits 1 with land_work_untraceable naming .atelier/issues/demo-7nnz.md, a record the branch never touched. That brings back the exact every-stop hook error the record is meant to remove, for a common setup (the user's own notes say local master here often lags origin). Suggested fix: compute changes against every candidate base ref (local head plus remote-tracking refs with the same name) and report only paths changed relative to all of them, or use the newest merge base among those refs; add a test with a stale local base and a newer remote-tracking base.
