---
schema: atelier.review
schema_version: 2
id: atelier-0908
title: finding on atelier-7tul @8095332a
target: atelier-7tul
identity: codex-plan-reviewer (gpt-6.1-sol)
commit: 8095332a5c72f40efe4006ed20d26b13e84f456b
created_at: 2026-11-09T02:07:15.695046389Z
event:
  event: finding
  recommendation: fix_now
---

high: .atelier/issues/atelier-7tul.md:36 Deleting scripts/merge-ci-passed.sh and requiring this item to merge through ci.passed cannot work in the planned single transition. merge.rs:883 reads the committed base policy, whose shell.check still executes scripts/merge-ci-passed.sh in the candidate merged checkout; deleting it makes that check exit 127. Installing the branch binary (line 44) changes neither the trusted policy nor the missing file. Split the transition: preserve the script while landing support and the new policy under the old check, then remove it in subsequent reviewed work after the base uses ci.passed. Prove the old-to-new transition with an integration test; do not bypass base-policy authority.
