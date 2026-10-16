---
schema: atelier.review
schema_version: 1
id: atelier-bulv
title: 'Checked fix b3076c97: the review_gate.rs error and reviews-and-merge.md now advise cherry-picking only code or planning commits, leaving out review and evidence records, and recording proof and review again. Checked this against evaluate_scope (an off-branch review bails) and evidence_freshness (an off-branch evidence pin never counts). The merge_gates test gates on evidence.attached, makes a fix and proof after the rebase, follows the advice, asserts FAIL review.current and FAIL evidence.attached, then records both again and merges. cargo nextest run --workspace passes 636/636. The only open finding is deferred: atelier-99k3, about mixed commits and the missing negative case.'
target: atelier-v9ve
identity: Independent reviewer (Claude subagent)
commit: 3f5e3cd9b7b091bb45f2789893844439aa38af45
created_at: 2026-10-16T20:08:45.458687956Z
event:
  event: approval
---

Checked fix b3076c97: the review_gate.rs error and reviews-and-merge.md now advise cherry-picking only code or planning commits, leaving out review and evidence records, and recording proof and review again. Checked this against evaluate_scope (an off-branch review bails) and evidence_freshness (an off-branch evidence pin never counts). The merge_gates test gates on evidence.attached, makes a fix and proof after the rebase, follows the advice, asserts FAIL review.current and FAIL evidence.attached, then records both again and merges. cargo nextest run --workspace passes 636/636. The only open finding is deferred: atelier-99k3, about mixed commits and the missing negative case.
