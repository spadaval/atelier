---
schema: atelier.review
schema_version: 1
id: atelier-jh2c
title: 'low: crates/atelier-cli/src/commands/merge_sync.rs:149 Any error inside the rerun loop is propagated with ''?'' after the merge commit already exists: split_command rejecting a hand-edited command, capture preflight or postflight refusals (reproduced: a passing rerun that leaves an untracked file makes capture refuse ''command changed code or policy'', so no receipt is written), or criteria that no longer resolve. The sync then ends in a bare ''merge: refused'' with no code and no approval report, and the receipts from earlier reruns stay uncommitted. A later --sync resumes from the merge''s first parent, so it can recover. Suggested fix: treat a per-receipt error as a sync_rerun_failed with its message, continue, still commit what was recorded and report the approval.'
target: atelier-s2ei
identity: Independent reviewer (Claude subagent)
commit: 87eab036a47ab1e9869463512458ac0ebd218a2f
created_at: 2026-10-22T22:28:45.689089530Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/merge_sync.rs:149 Any error inside the rerun loop is propagated with '?' after the merge commit already exists: split_command rejecting a hand-edited command, capture preflight or postflight refusals (reproduced: a passing rerun that leaves an untracked file makes capture refuse 'command changed code or policy', so no receipt is written), or criteria that no longer resolve. The sync then ends in a bare 'merge: refused' with no code and no approval report, and the receipts from earlier reruns stay uncommitted. A later --sync resumes from the merge's first parent, so it can recover. Suggested fix: treat a per-receipt error as a sync_rerun_failed with its message, continue, still commit what was recorded and report the approval.
