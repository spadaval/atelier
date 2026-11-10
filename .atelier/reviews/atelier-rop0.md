---
schema: atelier.review
schema_version: 2
id: atelier-rop0
title: finding on atelier-axfa @3ea213f4
target: atelier-axfa
identity: gpt-6.1-sol-r1
commit: 3ea213f49e9389be12b1ebd60a461c945d056156
created_at: 2026-11-10T22:32:37.525563592Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/commands.rs:38 A newly escalated question is invisible to its mission lead. Reproduced with the real next binary in a scratch repository with an item worktree and a base worktree: start an item in a ready mission led by lead1, then escalate as its implementer without a manual commit. Escalate tells the implementer that lead1 decides it and that the lead status lists the question, but status as lead1 on the base omits it, and show <mission> prints Questions: None. annotate_all receives branch-tip records from Repo::everything; the question exists only in the other checkout until submit, which itself refuses unanswered questions. The scenario test at next/crates/atelier-cli/tests/decisions.rs:479 hides this with an extra git commit before checking the lead views. Make the supported handoff expose the waiting question to the lead before the implementer waits, preserving the no-auto-commit contract, and test the actual escalate-to-lead sequence without that hidden commit. Contract: fail on Done when 7; no approval. Verdict: merge after this fix. Sizes: complexity high (decision state across the review lifecycle), value high (required disagreement exit), risk low (isolated, uninstalled next workspace). Fitness otherwise passes; added helpers, flags and state trace to requirements. Fresh proof: all 179 nextest tests pass, next build --locked --all-targets and clippy --all-targets with warnings denied pass, formatting and locked metadata pass in both workspaces, and both required last-commit and full own-change whitespace checks pass. A separate manual no-finding answer and --no-action decision carried through submit, review, approval and merge successfully. The destructive CI isolation job was not rerun.
