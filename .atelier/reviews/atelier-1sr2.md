---
schema: atelier.review
schema_version: 1
id: atelier-1sr2
title: The advice says to `git rm` the record files after `git cherry-pick -n`, but those files are newly staged, so plain `git rm <path>` refuses with 'the following file has changes staged in the index (use --cached to keep the file, or -f to force removal)'. Reproduced in a scratch repo. The test only passes because it uses `git rm -q -f`. Say `git rm -f` in the error text in crates/atelier-app/src/review_gate.rs and in docs/product/reviews-and-merge.md, and ideally give the listing command too (`git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence`).
target: atelier-imbo
identity: Independent reviewer (Claude subagent)
commit: 503453f194deb61db01ab1f93e29e7bd1f58fd62
created_at: 2026-10-16T20:47:42.181144387Z
event:
  event: finding
  recommendation: fix_now
---

The advice says to `git rm` the record files after `git cherry-pick -n`, but those files are newly staged, so plain `git rm <path>` refuses with 'the following file has changes staged in the index (use --cached to keep the file, or -f to force removal)'. Reproduced in a scratch repo. The test only passes because it uses `git rm -q -f`. Say `git rm -f` in the error text in crates/atelier-app/src/review_gate.rs and in docs/product/reviews-and-merge.md, and ideally give the listing command too (`git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence`).
