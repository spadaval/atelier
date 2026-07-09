---
created_at: "2026-07-06T17:38:22.560469639+00:00"
id: "atelier-id3y"
evidence_type: "test"
captured_at: "2026-07-06T17:38:19.649448340+00:00"
command: "cargo nextest run -p atelier-cli --test cli_integration bundle"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-tcai"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tcai"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli --test cli_integration bundle"
updated_at: "2026-07-06T17:38:26.228471306+00:00"
---

## Summary

cargo nextest run -p atelier-cli --test cli_integration bundle

## Command

```console
cargo nextest run -p atelier-cli --test cli_integration bundle
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1901
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.93s
────────────
 Nextest run ID 750ba907-4c45-464e-8e40-f424103b7ae4 with nextest profile: default
    Starting 11 tests across 1 binary (255 tests skipped)
        PASS [   0.150s] ( 1/11) atelier-cli::cli_integration issues::test_bundle_preview_rejects_duplicate_client_refs
        PASS [   0.153s] ( 2/11) atelier-cli::cli_integration mission_projection_worktree::test_bundle_rejects_removed_mission_resource_shape
        PASS [   0.156s] ( 3/11) atelier-cli::cli_integration mission_projection_worktree::test_bundle_apply_mid_apply_failure_leaves_canonical_files_unchanged
        PASS [   0.156s] ( 4/11) atelier-cli::cli_integration issues::test_bundle_preview_rejects_plan_and_milestone_resources
        PASS [   0.164s] ( 5/11) atelier-cli::cli_integration issues::test_bundle_preview_rejects_missing_client_ref
        PASS [   0.164s] ( 6/11) atelier-cli::cli_integration mission_projection_worktree::test_bundle_preview_rejects_duplicate_normalized_relationships_without_mutation
        PASS [   0.165s] ( 7/11) atelier-cli::cli_integration mission_projection_worktree::test_bundle_rejects_mission_parent_scope
        PASS [   0.171s] ( 8/11) atelier-cli::cli_integration issues::test_bundle_preview_rejects_status_outside_workflow_policy
        PASS [   0.378s] ( 9/11) atelier-cli::cli_integration issues::test_bundle_apply_accepts_configured_custom_issue_type
        PASS [   0.633s] (10/11) atelier-cli::cli_integration mission_projection_worktree::test_bundle_apply_records_links_export_and_rebuild
        PASS [   0.677s] (11/11) atelier-cli::cli_integration issues::test_bundle_apply_accepts_partial_issue_key_refs
────────────
     Summary [   0.678s] 11 tests run: 11 passed, 255 skipped
```

