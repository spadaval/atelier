---
schema: atelier.review
schema_version: 2
id: atelier-1sr2
title: The advice says to `git rm` the record files after `git cherry-pick -n`, but those files are newly staged, so plain `git rm <path>` refuses with 'the following file has changes staged in the index (use --cached to keep the file, or -f to force removal)'. Reproduced in a scratch repo. The test only passes because it uses `git rm -q -f`. Say `git rm -f` in the error text in crates/atelier-app/src/review_gate.rs and in docs/product/reviews-and-merge.md, and ideally give the listing command too (`git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence`).
target: atelier-imbo
identity: Independent reviewer (Claude subagent)
commit: 503453f194deb61db01ab1f93e29e7bd1f58fd62
created_at: 2026-10-16T20:47:42.181144387Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: b31f6595b38b9501ed3326d2fc3e883da3d0d7a8
    recorded_at: 2026-10-16T21:00:43.065728246Z
    resolution_commit: 6aedc323c5b8bc59d0ea602f0946c354a9955605
    reason: Error text and reviews-and-merge.md now advise git rm -f on each file listed by git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence. In a scratch repo, after git cherry-pick -n of a mixed commit (code + new review + new evidence file), the listing printed exactly the two added record files, plain git rm failed with 'changes staged in the index', git rm -f removed both, and the commit kept only the code change; the merge_gates test follows the same commands.
    migrated_from: atelier-8hng
---

The advice says to `git rm` the record files after `git cherry-pick -n`, but those files are newly staged, so plain `git rm <path>` refuses with 'the following file has changes staged in the index (use --cached to keep the file, or -f to force removal)'. Reproduced in a scratch repo. The test only passes because it uses `git rm -q -f`. Say `git rm -f` in the error text in crates/atelier-app/src/review_gate.rs and in docs/product/reviews-and-merge.md, and ideally give the listing command too (`git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence`).
