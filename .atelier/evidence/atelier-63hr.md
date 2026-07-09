---
created_at: "2026-07-09T15:28:58.640747067+00:00"
id: "atelier-63hr"
evidence_type: "test"
captured_at: "2026-07-09T15:28:47.822337158+00:00"
command: "cargo nextest run -p atelier-cli"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-gxq5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-gxq5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Full atelier-cli suite passes on the merge-preserving integration branch"
updated_at: "2026-07-09T15:28:58.643822552+00:00"
---

## Summary

Full atelier-cli suite passes on the merge-preserving integration branch

## Command

```console
cargo nextest run -p atelier-cli
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 56739
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/integrate-local-master/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.36s
────────────
 Nextest run ID d6edca08-42cc-4ae0-9656-3b2e67624090 with nextest profile: default
    Starting 461 tests across 4 binaries
        PASS [   0.010s] (  1/461) atelier-cli commands::create::tests::test_get_template_exists
        PASS [   0.013s] (  2/461) atelier-cli commands::create::tests::test_get_template_not_found
        PASS [   0.009s] (  3/461) atelier-cli commands::create::tests::test_invalid_priorities_never_validate
        PASS [   0.023s] (  4/461) atelier-cli commands::comment::tests::test_validate_known_kinds
        PASS [   0.024s] (  5/461) atelier-cli commands::comment::tests::test_validate_unknown_kinds
        PASS [   0.009s] (  6/461) atelier-cli commands::deps::tests::truncate_respects_limit
        PASS [   0.012s] (  7/461) atelier-cli commands::create::tests::test_validate_priority_invalid
        PASS [   0.013s] (  8/461) atelier-cli commands::create::tests::test_list_templates
        PASS [   0.013s] (  9/461) atelier-cli commands::create::tests::test_validate_priority_valid
        PASS [   0.014s] ( 10/461) atelier-cli commands::create::tests::test_validate_priority_malicious
        PASS [   0.015s] ( 11/461) atelier-cli commands::create::tests::test_template_bug_description_prefix
        PASS [   0.017s] ( 12/461) atelier-cli commands::forgejo::tests::inspect_roles_reports_success_and_collapsed_mappings
        PASS [   0.014s] ( 13/461) atelier-cli commands::import::tests::test_import_beads_preserves_notes_as_activity_records
        PASS [   0.013s] ( 14/461) atelier-cli commands::import::tests::test_imported_beads_description_uses_current_issue_sections
        PASS [   0.028s] ( 15/461) atelier-cli commands::create::tests::test_template_feature_description_prefix
        PASS [   0.018s] ( 16/461) atelier-cli commands::import::tests::test_import_beads_fixture_preserves_counts_and_links
        PASS [   0.029s] ( 17/461) atelier-cli commands::create::tests::test_unknown_template_returns_none
        PASS [   0.018s] ( 18/461) atelier-cli commands::import::tests::test_import_staging_failure_leaves_no_records_or_activities_and_allows_retry
        PASS [   0.018s] ( 19/461) atelier-cli commands::import::tests::test_import_rejects_late_invalid_record_without_partial_files_and_allows_retry
        PASS [   0.032s] ( 20/461) atelier-cli commands::create::tests::test_template_fields
        PASS [   0.130s] ( 21/461) atelier-cli commands::comment::tests::test_comment_roundtrip_order_and_missing_issue
        PASS [   0.131s] ( 22/461) atelier-cli commands::comment::tests::test_add_comment_to_nonexistent_issue
        PASS [   0.131s] ( 23/461) atelier-cli commands::comment::tests::test_add_unicode_comment
        PASS [   0.132s] ( 24/461) atelier-cli commands::comment::tests::test_add_comment_with_newlines
        PASS [   0.135s] ( 25/461) atelier-cli commands::comment::tests::test_add_multiple_comments
        PASS [   0.139s] ( 26/461) atelier-cli commands::comment::tests::test_add_comment_to_existing_issue
        PASS [   0.140s] ( 27/461) atelier-cli commands::comment::tests::test_comment_kind_roundtrip
        PASS [   0.013s] ( 28/461) atelier-cli commands::pr::tests::render_comment_lines_filters_resolved_comments
        PASS [   0.013s] ( 29/461) atelier-cli commands::pr::tests::render_pull_comment_lines_include_body_summary
        PASS [   0.146s] ( 30/461) atelier-cli commands::comment::tests::test_comment_with_null_bytes
        PASS [   0.147s] ( 31/461) atelier-cli commands::comment::tests::test_add_comment_sql_injection
        PASS [   0.152s] ( 32/461) atelier-cli commands::comment::tests::test_add_empty_comment
        PASS [   0.158s] ( 33/461) atelier-cli commands::comment::tests::test_invalid_comment_kind_is_rejected
        PASS [   0.162s] ( 34/461) atelier-cli commands::comment::tests::test_add_comment_with_special_chars
        PASS [   0.153s] ( 35/461)
```
