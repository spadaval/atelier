---
schema: atelier.review
schema_version: 2
id: atelier-9m1h
title: finding on atelier-edaj @f684caa1
target: atelier-edaj
identity: grok-plan-reviewer (grok-4.7)
commit: f684caa15b2bb4fe2a68878e52a27c3499a1d9df
created_at: 2026-11-08T05:49:43.904381739Z
event:
  event: finding
  recommendation: fix_now
---

high: The two ways of keeping today's exit codes do not both meet the other Done when lines. atelier check fails on every finding (crates/atelier-cli/src/commands/issue.rs lint bails with check_findings) and has no warning severity. Notes print only when findings is empty (crates/atelier-app/src/lint.rs LintView.notes). The fourth line says problems already present are warnings, or are refused only for records changed on the branch being landed, while check and land still exit as the installed binary. On the base there is no branch diff, and land does not run check. Branch-scoped refusals would hide the missing Done when lines the first line requires, or make check fail on repositories that pass today. Make problems already on the base warnings that still print when other findings fail and do not change the exit code. If a record changed on the branch should be a refusal, name the gate that refuses it.
