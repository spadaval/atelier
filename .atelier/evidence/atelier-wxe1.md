---
created_at: "2026-07-06T18:31:48.926353659+00:00"
id: "atelier-wxe1"
evidence_type: "validation"
captured_at: "2026-07-06T18:31:43.494149723+00:00"
command: "cargo nextest run -p atelier-app -p atelier-cli pr::tests::derived_open_context_uses_the_branch_owner_and_workflow_branches pr::tests::parse_pull_request_reference_accepts_number_and_matching_url pr::tests::parse_pull_request_reference_rejects_mismatched_url_context pr::tests::pr_link_fetches_remote_pull_and_persists_owner_field pr::tests::pr_comment_posts_to_linked_pull_and_records_owner_action pr::tests::pr_review_posts_review_event_and_records_owner_action forgejo::tests::provider_contract_accepts_only_official_approved_event forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id review_room::tests::room_merge_requires_current_approval_and_resolved_blocking_findings commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret commands::workflow::tests::review_open_action_persists_room_review_field provider_request_review_pushes_source_before_opening_pr provider_review_open_action_reads_workflow_config_and_global_secret request_review_preserves_review_artifact_field review_surface_derives_open_context_and_uses_submit_and_show review_help_exposes_only_the_collapsed_public_contract smoke::lifecycle::test_dependency_chain_and_ready"
exit_status: "100"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent focused review-surface validation: derived direct/workflow open context; provider/native-room parity; existing-link recovery; live comment protocol; collapsed submit decisions; official APPROVED serialization; resolve/merge behavior; retired help aliases."
updated_at: "2026-07-06T18:31:53.779873649+00:00"
---

## Summary

Independent focused review-surface validation: derived direct/workflow open context; provider/native-room parity; existing-link recovery; live comment protocol; collapsed submit decisions; official APPROVED serialization; resolve/merge behavior; retired help aliases.

## Command

```console
cargo nextest run -p atelier-app -p atelier-cli pr::tests::derived_open_context_uses_the_branch_owner_and_workflow_branches pr::tests::parse_pull_request_reference_accepts_number_and_matching_url pr::tests::parse_pull_request_reference_rejects_mismatched_url_context pr::tests::pr_link_fetches_remote_pull_and_persists_owner_field pr::tests::pr_comment_posts_to_linked_pull_and_records_owner_action pr::tests::pr_review_posts_review_event_and_records_owner_action forgejo::tests::provider_contract_accepts_only_official_approved_event forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id review_room::tests::room_merge_requires_current_approval_and_resolved_blocking_findings commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret commands::workflow::tests::review_open_action_persists_room_review_field provider_request_review_pushes_source_before_opening_pr provider_review_open_action_reads_workflow_config_and_global_secret request_review_preserves_review_artifact_field review_surface_derives_open_context_and_uses_submit_and_show review_help_exposes_only_the_collapsed_public_contract smoke::lifecycle::test_dependency_chain_and_ready
```

Exit status: 100

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 11469
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.96s
────────────
 Nextest run ID b27c6d91-1dee-43a7-853f-b93ef216089b with nextest profile: default
    Starting 17 tests across 5 binaries (537 tests skipped)
        PASS [   0.009s] ( 1/17) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.011s] ( 2/17) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.012s] ( 3/17) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.013s] ( 4/17) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        FAIL [   0.027s] ( 5/17) atelier-cli commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret
  stdout ───

    running 1 test
    test commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret ... FAILED

    failures:

    failures:
        commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 137 filtered out; finished in 0.00s

  stderr ───

    thread 'commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret' (1580524) panicked at crates/atelier-cli/src/commands/workflow.rs:2749:9:
    assertion `left == right` failed
      left: 0
     right: 1
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.073s] ( 6/17) atelier-cli::cli_integration review_help_exposes_only_the_collapsed_public_contract
        PASS [   0.145s] ( 7/17) atelier-app pr::tests::pr_comment_posts_to_linked_pull_and_records_owner_action
        PASS [   0.148s] ( 8/17) atelier-app pr::tests::pr_review_posts_review_event_and_records_owner_action
        PASS [   0.149s] ( 9/17) atelier-app pr::tests::derived_open_context_uses_the_branch_owner_and_workflow_branches
        PASS [   0.303s] (10/17) atelier-cli commands::workflow::tests::review_open_action_persists_room_review_field
        PASS [   0.304s] (11/17) atelier-app pr::tests::pr_link_fetches_remote_pull_and_persists_owner_field
        FAIL [   0.823s] (12/17) atelier-cli::cli_integration provider_review_open_action_reads_workflow_config_and_global_secret
  stdout ───

    running 1 test
    test provider_review_open_action_reads_workflow_config_and_global_secret ... FAILED

    failures:

    failures:
        provider_review_open_action_reads_workflow_config_and_global_secret

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 272 filtered out; finished in 0.81s

  stderr ───

    thread 'provider_review_open_action_reads_workflow_config_and_global_secret' (1580477) panicked at crates/atelier-cli/tests/cli_integration.rs:1110:5:
    Issue Transitions atelier-sr20 - Provider epic
    ==============================================
    State
    -----
    Status:   in_progress
    Type:     epic
    Options:  3
    Branch Context
    --------------
    Owner:    epic atelier-sr20 (epic)
    Source:   epic/atelier-sr20
    Base:     main
    Target:   main
    Current:  epic/atelier-sr20
    State:    tracker changes present: 4 paths:  M .atelier/issues/atelier-sr20.md, ?? .atelier/issues/atelier-sr20.activity/20260706T183146177056Z.md, ?? .atelier/issues/atelier-sr20.activity/20260706T183146177230Z.md, 1 more omitted

    block [allowed]
      From: todo, in_progress, review, validation
      To:   blocked
      Requirements: satisfied
      Decision: allowed
    Validators
    ----------
    (none)
    Planned Actions
    ---------------
    (none)
    Description
    -----------
      Mark work blocked while preserving current proof expectations.
    Commands
    --------
      atelier issue transition atelier-sr20 block

    request_revie
```
