---
created_at: "2026-06-20T05:10:17.010953378+00:00"
id: "atelier-5op0"
evidence_type: "validation"
captured_at: "2026-06-20T05:10:17.010946522+00:00"
target:
  kind: "issue"
  id: "atelier-wehh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wehh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Implemented provider-owned terminal action vocabulary and planning: .atelier/workflow.yaml and STARTER_POLICY_YAML close epic/validation workflows with tracker.commit, branch.push, review.merge, base.sync, while branch_integrate is absent. Validation passed: target/debug/atelier workflow check; cargo test -p atelier-workflow action --lib; cargo test -p atelier-cli commands::workflow::tests::provider_terminal_actions_plan_without_local_branch_integrate --lib; cargo nextest run -p atelier-cli --test cli_integration -E 'test(test_branch_actions_prepare_and_integrate_epic_workflow) | test(test_branch_integrate_action_failure_rolls_back_status_with_recovery)'; cargo fmt -- --check; cargo check -p atelier-cli; git diff --check."
updated_at: "2026-06-20T05:11:09.233396908+00:00"
---

Implemented provider-owned terminal action vocabulary and planning: .atelier/workflow.yaml and STARTER_POLICY_YAML close epic/validation workflows with tracker.commit, branch.push, review.merge, base.sync, while branch_integrate is absent. Validation passed: target/debug/atelier workflow check; cargo test -p atelier-workflow action --lib; cargo test -p atelier-cli commands::workflow::tests::provider_terminal_actions_plan_without_local_branch_integrate --lib; cargo nextest run -p atelier-cli --test cli_integration -E 'test(test_branch_actions_prepare_and_integrate_epic_workflow) | test(test_branch_integrate_action_failure_rolls_back_status_with_recovery)'; cargo fmt -- --check; cargo check -p atelier-cli; git diff --check.
