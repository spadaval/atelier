---
schema: atelier.review
schema_version: 2
id: atelier-lsoh
title: finding on atelier-axfa @9ed19231
target: atelier-axfa
identity: gpt-6.1-sol-r2
commit: 9ed19231a8a4f813ed4a6ed4813eabf7b6a665d4
created_at: 2026-11-10T22:37:35.267171975Z
event:
  event: finding
  recommendation: fix_now
---

low: next/crates/atelier-cli/src/commands.rs:38 The implementer loses the waiting question and approval time in status from the base checkout. Reproduced with next/target/debug/atelier in a scratch repository with item and base worktrees: start demo-ab12 as t1, escalate "Should it be louder?", commit the question, then run status as t1 in the base worktree. It prints only "demo-ab12 (building): Say hello", with no question or decider wait. After decide, submit, review and verdict approve, the same status prints only "demo-ab12 (approved): Say hello", while status on the item branch prints "approved since <verdict commit time>". The overview annotates only overview.led, and output.rs:380 renders held entries without either field. This misses Done when 2 and 8 when a thread resumes on the base or holds several items: the state that identifies a lost wait is absent even though the question and approval are committed and readable. Populate and render the question/wait and approval metadata for overview.held too, retaining the branch-switch next step, and cover both observations from a base worktree. Verdict: merge after this fix, with approval withheld. Sizes: complexity high (decision state crosses commands and review rounds), value high (required third disagreement exit), risk low (isolated, uninstalled next workspace). Contract: fail on the status requirements; other requirements and the earlier atelier-rop0 handoff fix pass inspection and scenarios. Fitness: pass apart from this omission; the added types, flags and helpers trace to requirements. Fresh proof at the pinned head: all 179 nextest tests passed with build-jobs 4 and test-threads 4; clippy --all-targets -j 4 with warnings denied passed; formatting and locked metadata passed in both workspaces; the requested last-commit whitespace check and whitespace checks on all nine own commits passed. No code was edited. Scratch reproduction removed. Open questions: none. Residual risk: the destructive CI isolation job and cross-repository smoke test were not rerun; current tests do not cover implementer status from the base for these new fields.
