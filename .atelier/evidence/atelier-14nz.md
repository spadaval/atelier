---
created_at: "2026-06-13T19:51:50.895580491+00:00"
id: "atelier-14nz"
evidence_type: "validation"
captured_at: "2026-06-13T19:51:50.895507092+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fyms"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Independent rerun for atelier-fyms after atelier-eovw. Supersedes failed evidence atelier-76uy and atelier-tbxq: the prior new-issue initial-status failure is no longer observed. Classifications: starter policy PASS (workflow init creates .atelier/workflow.yaml with standard_review_proof and lightweight_spike); workflow check PASS (current repo reports Policy pass, Record Health pass, 343 issues checked; temp migrated repos report pass); status migration PASS (legacy open -> todo, closed -> done preserving closed_at, archived -> archived preserving closed_at); issue creation after migration PASS (new issue shows status/category todo/todo and start allowed); start transition PASS (root start and issue transition start move todo/blocked to in_progress); blocked transition PASS (request_validation from in_progress fails on review_ready and records/display blockers); close with evidence PASS (standard issue close is blocked without proof/clean worktree, then passes after attached pass evidence and clean temp commit); lightweight spike close PASS (spike closes from review to done with close reason and no evidence requirement); archive PASS (policy with archive transition rejects ambiguous close without --to and closes to archived with --to archived); missing .atelier/workflow.yaml PASS (workflow check and transition options fail with workflow_config_missing); unmigrated-record failures PASS (workflow check rejects open as invalid and transition says run workflow migrate-statuses); normal reliance on raw workflow validate ABSENT/PASS (normal help/status surfaces advertise workflow init/check/migrate-statuses, start, issue transition, and issue close; no normal workflow validate path used as proof). Focused tests PASS: cargo test --test cli_integration test_issue_create_after_workflow_migration_uses_configured_initial_status -- --nocapture; cargo test --test cli_integration test_workflow_migrate_statuses_rewrites_legacy_issue_statuses_and_preserves_close_metadata -- --nocapture. Required checks PASS: target/debug/atelier workflow check, lint, export --check, doctor, git diff --check. Residual risks: full nextest/extended suite not run for this bounded validation; cargo test emitted existing dead-code warnings; current worktree is dirty only from atelier-fyms claim/start/evidence activity plus this evidence record. Follow-up IDs: none recommended from this rerun."
updated_at: "2026-06-13T19:51:53.143858079+00:00"
---

Independent rerun for atelier-fyms after atelier-eovw. Supersedes failed evidence atelier-76uy and atelier-tbxq: the prior new-issue initial-status failure is no longer observed. Classifications: starter policy PASS (workflow init creates .atelier/workflow.yaml with standard_review_proof and lightweight_spike); workflow check PASS (current repo reports Policy pass, Record Health pass, 343 issues checked; temp migrated repos report pass); status migration PASS (legacy open -> todo, closed -> done preserving closed_at, archived -> archived preserving closed_at); issue creation after migration PASS (new issue shows status/category todo/todo and start allowed); start transition PASS (root start and issue transition start move todo/blocked to in_progress); blocked transition PASS (request_validation from in_progress fails on review_ready and records/display blockers); close with evidence PASS (standard issue close is blocked without proof/clean worktree, then passes after attached pass evidence and clean temp commit); lightweight spike close PASS (spike closes from review to done with close reason and no evidence requirement); archive PASS (policy with archive transition rejects ambiguous close without --to and closes to archived with --to archived); missing .atelier/workflow.yaml PASS (workflow check and transition options fail with workflow_config_missing); unmigrated-record failures PASS (workflow check rejects open as invalid and transition says run workflow migrate-statuses); normal reliance on raw workflow validate ABSENT/PASS (normal help/status surfaces advertise workflow init/check/migrate-statuses, start, issue transition, and issue close; no normal workflow validate path used as proof). Focused tests PASS: cargo test --test cli_integration test_issue_create_after_workflow_migration_uses_configured_initial_status -- --nocapture; cargo test --test cli_integration test_workflow_migrate_statuses_rewrites_legacy_issue_statuses_and_preserves_close_metadata -- --nocapture. Required checks PASS: target/debug/atelier workflow check, lint, export --check, doctor, git diff --check. Residual risks: full nextest/extended suite not run for this bounded validation; cargo test emitted existing dead-code warnings; current worktree is dirty only from atelier-fyms claim/start/evidence activity plus this evidence record. Follow-up IDs: none recommended from this rerun.
