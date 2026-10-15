---
schema: atelier.review
schema_version: 1
id: atelier-s1wo
title: 'crates/atelier-cli/src/commands/issue.rs:840: issue update --body only checks section parse/required-for-type, not the record rules atelier check enforces, so it writes records that then fail check. Repro (scratch repo): epic created with Outcome+Proof, transitioned ready (todo), then ''issue update <epic> --body $"## Outcome\nO2"'' succeeds and ''atelier check'' fails with ''Ready epic requires Proof''; likewise a mission''s Success criteria can be dropped (''Mission requires Success criteria or a linked strategy''). This violates the constraint that update --body is validated the same as atelier check. Fix: before write_issue_atomic, run the same per-record lint used by check (lint.rs:136-188 section/Proof/Success criteria rules) on the updated record and refuse on any finding; add tests for ready-epic Proof removal and mission Success criteria removal.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:43.731294556Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/issue.rs:840: issue update --body only checks section parse/required-for-type, not the record rules atelier check enforces, so it writes records that then fail check. Repro (scratch repo): epic created with Outcome+Proof, transitioned ready (todo), then 'issue update <epic> --body $"## Outcome\nO2"' succeeds and 'atelier check' fails with 'Ready epic requires Proof'; likewise a mission's Success criteria can be dropped ('Mission requires Success criteria or a linked strategy'). This violates the constraint that update --body is validated the same as atelier check. Fix: before write_issue_atomic, run the same per-record lint used by check (lint.rs:136-188 section/Proof/Success criteria rules) on the updated record and refuse on any finding; add tests for ready-epic Proof removal and mission Success criteria removal.
