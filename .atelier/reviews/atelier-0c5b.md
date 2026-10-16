---
schema: atelier.review
schema_version: 1
id: atelier-0c5b
title: 'Reviewed master...HEAD for crates and docs. merge_diverged (merge.rs:72) now advises git merge <base> on the work branch, fresh proof, and no rebase once review records exist; this matches land''s behind-base note (session.rs:350-351). The pins-outside error (review_gate.rs:417) names rebase/reset as the likely cause and gives reflog reset plus merge-base as the recovery. rebasing_a_reviewed_branch_explains_the_cause_and_recovery goes through approve, base advance, rebase, refusal, reset to the reviewed tip, git merge main, and a successful atelier merge. The extended divergence test checks the advice text. Both tests pass locally. reviews-and-merge.md and workflow-configuration.md agree with each other and with the messages. Two non-blocking findings: atelier-eix5 (defer), atelier-91va (no_action).'
target: atelier-j48s
identity: Independent reviewer (Claude subagent)
commit: 06f62abff594387257edc29e1fd681eb9afacd15
created_at: 2026-10-16T18:47:25.090804002Z
event:
  event: approval
---

Reviewed master...HEAD for crates and docs. merge_diverged (merge.rs:72) now advises git merge <base> on the work branch, fresh proof, and no rebase once review records exist; this matches land's behind-base note (session.rs:350-351). The pins-outside error (review_gate.rs:417) names rebase/reset as the likely cause and gives reflog reset plus merge-base as the recovery. rebasing_a_reviewed_branch_explains_the_cause_and_recovery goes through approve, base advance, rebase, refusal, reset to the reviewed tip, git merge main, and a successful atelier merge. The extended divergence test checks the advice text. Both tests pass locally. reviews-and-merge.md and workflow-configuration.md agree with each other and with the messages. Two non-blocking findings: atelier-eix5 (defer), atelier-91va (no_action).
