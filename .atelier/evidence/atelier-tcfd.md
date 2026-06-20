---
created_at: "2026-06-20T13:22:42.855175961+00:00"
id: "atelier-tcfd"
evidence_type: "validation"
captured_at: "2026-06-20T13:22:42.855157133+00:00"
target:
  kind: "issue"
  id: "atelier-wee2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wee2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Provider review.complete now gates on approved open Forgejo PRs before terminal merge, reviewer guidance exposes approve/request-changes, and docs define the approval-before-close process. Validation passed: cargo fmt -- --check; git diff --check; cargo test -p atelier-app forgejo::tests::lists_pull_reviews --lib; cargo nextest run -p atelier-cli -E 'test(commands::workflow::tests::provider_review_complete_accepts_approved_open_pr_before_merge) | test(commands::workflow::tests::provider_review_complete_blocks_unapproved_changed_or_merged_prs) | test(commands::workflow::tests::linked_pr_merged_validator_reports_required_states)'; cargo nextest run -p atelier-cli -E 'test(commands::workflow::tests::provider_terminal_actions_plan_without_local_branch_integrate) | test(commands::workflow::tests::action_preflight_checks_git_actions_before_execution) | test(commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_env_secret)'; cargo nextest run -p atelier-cli --test cli_integration -E 'test(setup_guidance::test_man_lists_roles)'; cargo check -p atelier-cli; target/debug/atelier workflow check; target/debug/atelier lint atelier-wee2."
updated_at: "2026-06-20T13:22:47.375369278+00:00"
---

Provider review.complete now gates on approved open Forgejo PRs before terminal merge, reviewer guidance exposes approve/request-changes, and docs define the approval-before-close process. Validation passed: cargo fmt -- --check; git diff --check; cargo test -p atelier-app forgejo::tests::lists_pull_reviews --lib; cargo nextest run -p atelier-cli -E 'test(commands::workflow::tests::provider_review_complete_accepts_approved_open_pr_before_merge) | test(commands::workflow::tests::provider_review_complete_blocks_unapproved_changed_or_merged_prs) | test(commands::workflow::tests::linked_pr_merged_validator_reports_required_states)'; cargo nextest run -p atelier-cli -E 'test(commands::workflow::tests::provider_terminal_actions_plan_without_local_branch_integrate) | test(commands::workflow::tests::action_preflight_checks_git_actions_before_execution) | test(commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_env_secret)'; cargo nextest run -p atelier-cli --test cli_integration -E 'test(setup_guidance::test_man_lists_roles)'; cargo check -p atelier-cli; target/debug/atelier workflow check; target/debug/atelier lint atelier-wee2.
