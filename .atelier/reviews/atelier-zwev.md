---
schema: atelier.review
schema_version: 1
id: atelier-zwev
title: 'Re-reviewed fix 6aedc323 for atelier-1sr2 and atelier-swt9. Ran the advised commands verbatim in a scratch repo: git cherry-pick -n of a commit mixing code with a new review file and a new evidence file; the git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence listing printed exactly the two record files; git rm -f on each removed them (plain git rm fails, confirming the original defect); the commit kept only the code; git merge of the base then succeeded. The error text in review_gate.rs is numbered reset, cherry-pick without records, merge base, record proof and review again; reviews-and-merge.md gives the same steps in the same order with the same commands. The merge_gates test asserts the steps and follows them to a successful merge after fresh proof and approval. cargo nextest run --workspace: 644 passed. Note: the listing pathspecs are relative to the current directory, so the commands assume the repo root, as all the advice does.'
target: atelier-imbo
identity: Independent reviewer (Claude subagent)
commit: b31f6595b38b9501ed3326d2fc3e883da3d0d7a8
created_at: 2026-10-16T21:00:55.589998651Z
event:
  event: approval
---

Re-reviewed fix 6aedc323 for atelier-1sr2 and atelier-swt9. Ran the advised commands verbatim in a scratch repo: git cherry-pick -n of a commit mixing code with a new review file and a new evidence file; the git diff --cached --name-only --diff-filter=A -- .atelier/reviews .atelier/evidence listing printed exactly the two record files; git rm -f on each removed them (plain git rm fails, confirming the original defect); the commit kept only the code; git merge of the base then succeeded. The error text in review_gate.rs is numbered reset, cherry-pick without records, merge base, record proof and review again; reviews-and-merge.md gives the same steps in the same order with the same commands. The merge_gates test asserts the steps and follows them to a successful merge after fresh proof and approval. cargo nextest run --workspace: 644 passed. Note: the listing pathspecs are relative to the current directory, so the commands assume the repo root, as all the advice does.
