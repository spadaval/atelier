---
created_at: "2026-07-06T18:40:44.346696930+00:00"
id: "atelier-zgah"
evidence_type: "test"
captured_at: "2026-07-06T18:40:40.211170571+00:00"
command: "cargo nextest run -p atelier-app -p atelier-cli -E 'test(/mutation_paths_have_no_eager_cache_refresh_helper|test_bundle_apply_records_links_export_and_rebuild|test_issue_mutations_leave_stale_cache_for_one_lazy_repair|test_evidence_capture_records_command_metadata_and_attaches_targets|test_issue_transition_options_and_successful_execution_follow_workflow_policy|room_merge_requires_current_approval_and_resolved_blocking_findings|persist_pull_request_writes_owner_epic_field_and_child_inherits/)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-5m81"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5m81"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Eager refresh removal, batch writes, and one later lazy repair"
updated_at: "2026-07-06T18:40:47.889852215+00:00"
---

## Summary

Eager refresh removal, batch writes, and one later lazy repair

## Command

```console
cargo nextest run -p atelier-app -p atelier-cli -E 'test(/mutation_paths_have_no_eager_cache_refresh_helper|test_bundle_apply_records_links_export_and_rebuild|test_issue_mutations_leave_stale_cache_for_one_lazy_repair|test_evidence_capture_records_command_metadata_and_attaches_targets|test_issue_transition_options_and_successful_execution_follow_workflow_policy|room_merge_requires_current_approval_and_resolved_blocking_findings|persist_pull_request_writes_owner_epic_field_and_child_inherits/)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1468
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-x7lq/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/root/.codex/worktrees/mska-x7lq/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
────────────
 Nextest run ID 081c6bf9-04a5-48b5-b88d-350eb05c67dd with nextest profile: default
    Starting 7 tests across 5 binaries (562 tests skipped)
        PASS [   0.014s] (1/7) atelier-cli::bin/atelier cache_acquisition_tests::mutation_paths_have_no_eager_cache_refresh_helper
        PASS [   0.233s] (2/7) atelier-app pr::tests::persist_pull_request_writes_owner_epic_field_and_child_inherits
        PASS [   0.236s] (3/7) atelier-app review_room::tests::room_merge_requires_current_approval_and_resolved_blocking_findings
        PASS [   0.575s] (4/7) atelier-cli::cli_integration mission_projection_worktree::test_bundle_apply_records_links_export_and_rebuild
        PASS [   0.702s] (5/7) atelier-cli::cli_integration setup_guidance::test_issue_transition_options_and_successful_execution_follow_workflow_policy
        PASS [   1.514s] (6/7) atelier-cli::cli_integration issues::test_issue_mutations_leave_stale_cache_for_one_lazy_repair
        PASS [   1.746s] (7/7) atelier-cli::cli_integration records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets
────────────
     Summary [   1.747s] 7 tests run: 7 passed, 562 skipped
```
