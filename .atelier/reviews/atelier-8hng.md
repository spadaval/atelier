---
schema: atelier.review
schema_version: 1
id: atelier-8hng
title: Error text and reviews-and-merge.md now advise git rm -f on each file listed by git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence. In a scratch repo, after git cherry-pick -n of a mixed commit (code + new review + new evidence file), the listing printed exactly the two added record files, plain git rm failed with 'changes staged in the index', git rm -f removed both, and the commit kept only the code change; the merge_gates test follows the same commands.
target: atelier-imbo
identity: Independent reviewer (Claude subagent)
commit: b31f6595b38b9501ed3326d2fc3e883da3d0d7a8
created_at: 2026-10-16T21:00:43.065728246Z
event:
  event: disposition
  finding: atelier-1sr2
  decision: fix_now
  supersedes: null
  resolution_commit: 6aedc323c5b8bc59d0ea602f0946c354a9955605
  follow_up: null
---

Error text and reviews-and-merge.md now advise git rm -f on each file listed by git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence. In a scratch repo, after git cherry-pick -n of a mixed commit (code + new review + new evidence file), the listing printed exactly the two added record files, plain git rm failed with 'changes staged in the index', git rm -f removed both, and the commit kept only the code change; the merge_gates test follows the same commands.
