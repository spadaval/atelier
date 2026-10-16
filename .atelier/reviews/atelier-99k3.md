---
schema: atelier.review
schema_version: 1
id: atelier-99k3
title: 'The advice is worded per commit (''cherry-pick code or planning commits ... not review or evidence records''), but one post-rebase commit can hold both, e.g. a planning commit that closes the issue and was committed with git add -A after recording evidence or a review. Issue files themselves never carry review or evidence data (those live in .atelier/reviews and .atelier/evidence), so pure planning commits are fine. Cherry-picking a mixed commit that adds a .atelier/reviews file brings back the same ''pins a commit outside the work branch'' refusal, and a mixed evidence file adds a record that never counts. Say to drop .atelier/reviews and .atelier/evidence files from such commits (for example git cherry-pick -n, then git restore --staged --worktree on those files). Related coverage gap: the test never cherry-picks a record, so nothing shows that doing so fails. The advice is conservative and still works, so this does not need fixing now.'
target: atelier-v9ve
identity: Independent reviewer (Claude subagent)
commit: 3f5e3cd9b7b091bb45f2789893844439aa38af45
created_at: 2026-10-16T20:08:38.946673402Z
event:
  event: finding
  recommendation: defer
---

The advice is worded per commit ('cherry-pick code or planning commits ... not review or evidence records'), but one post-rebase commit can hold both, e.g. a planning commit that closes the issue and was committed with git add -A after recording evidence or a review. Issue files themselves never carry review or evidence data (those live in .atelier/reviews and .atelier/evidence), so pure planning commits are fine. Cherry-picking a mixed commit that adds a .atelier/reviews file brings back the same 'pins a commit outside the work branch' refusal, and a mixed evidence file adds a record that never counts. Say to drop .atelier/reviews and .atelier/evidence files from such commits (for example git cherry-pick -n, then git restore --staged --worktree on those files). Related coverage gap: the test never cherry-picks a record, so nothing shows that doing so fails. The advice is conservative and still works, so this does not need fixing now.
