---
schema: atelier.review
schema_version: 1
id: atelier-6ajc
title: finding on atelier-0uaj @46a269e9
target: atelier-0uaj
identity: Independent reviewer (Claude Sonnet subagent)
commit: 46a269e98ef40a08ce787c618d8dd0f7b5e42bee
created_at: 2026-10-23T15:58:51.939179985Z
event:
  event: finding
  recommendation: defer
---

medium: crates/atelier-cli/src/commands/session.rs (records_uncommitted path) and merge.rs owned(): when a done claim exists only in uncommitted records (issue closed, not committed), gates read HEAD, so merge --check prints 'work.terminal: in_progress, not closed. Next: atelier issue transition <id> close --reason …'. Running that fix verbatim fails ('Transition close is not available from status done'). Reverse case (HEAD done, working copy reopened) prints '✓ work.terminal' beside the refusal. Verdict stays refused (safe), but a printed fix does not run. Suggest: when records are uncommitted, either omit the fixes of record-state gates or say the gates were read from HEAD.
