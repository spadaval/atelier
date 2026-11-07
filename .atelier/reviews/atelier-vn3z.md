---
schema: atelier.review
schema_version: 2
id: atelier-vn3z
title: 'crates/atelier-cli/src/commands/workflow.rs:146 — under -q a refused transition now drops the validator Hint lines (and the Command: retry line) entirely instead of moving them to stderr. The bail message at :157 carries only ''validator X failed: <reason>'', never ValidatorResult.help, so the recovery step is lost for exactly the scripts and agents -q serves. Reproduced: a decision without a Decision section, `atelier -q issue transition <id> decide --reason x` exits 1 with stderr ''validator decision.recorded failed: Decision section is missing'' and no hint, while the loud run shows ''Hint: Add a ## Decision section ... with `atelier issue update <id> --body`''. The same loss hits evidence.attached (''Record fresh proof with `atelier evidence record --target issue/<id> ...`''), the most common close refusal, and blockers.transitive_none_open (''Inspect the blocker with ...''). Before this change a -q caller still received the hint (on stdout); now it receives it nowhere, contrary to standards.md''s ''actionable errors'' and the issue''s ''explains itself on stderr''. Smallest fix: append each failed validator''s hint to the error (or render the report to stderr when quiet), and assert the hint in quiet_tokens.rs, which currently checks only blocker text.'
target: atelier-fnak
identity: Independent reviewer (Claude subagent)
commit: 76b35212b3cc5d0a5da2e99b331af58cb54dd53f
created_at: 2026-10-17T09:46:16.127169138Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 040890bab0ef276341ae519142bd7d782c26e4aa
    recorded_at: 2026-10-17T09:50:32.710337602Z
    resolution_commit: 3afd65a9a4910ecad0da4395e5de751f3d524de6
    reason: The refusal error ends with each failed validator's hint as its Next step, on stderr in every mode; tests cover blocked start and missing-evidence close.
    migrated_from: atelier-rvxi
---

crates/atelier-cli/src/commands/workflow.rs:146 — under -q a refused transition now drops the validator Hint lines (and the Command: retry line) entirely instead of moving them to stderr. The bail message at :157 carries only 'validator X failed: <reason>', never ValidatorResult.help, so the recovery step is lost for exactly the scripts and agents -q serves. Reproduced: a decision without a Decision section, `atelier -q issue transition <id> decide --reason x` exits 1 with stderr 'validator decision.recorded failed: Decision section is missing' and no hint, while the loud run shows 'Hint: Add a ## Decision section ... with `atelier issue update <id> --body`'. The same loss hits evidence.attached ('Record fresh proof with `atelier evidence record --target issue/<id> ...`'), the most common close refusal, and blockers.transitive_none_open ('Inspect the blocker with ...'). Before this change a -q caller still received the hint (on stdout); now it receives it nowhere, contrary to standards.md's 'actionable errors' and the issue's 'explains itself on stderr'. Smallest fix: append each failed validator's hint to the error (or render the report to stderr when quiet), and assert the hint in quiet_tokens.rs, which currently checks only blocker text.
