---
created_at: "2026-06-23T23:46:52.869930991+00:00"
id: "atelier-khxq"
evidence_type: "validation"
captured_at: "2026-06-23T23:46:13.801644241+00:00"
command: "sh -c 'cargo fmt -- --check && cargo check -p atelier-cli && cargo nextest run -p atelier-cli && target/debug/atelier lint atelier-c0qc && target/debug/atelier export --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-kx2y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kx2y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Human CLI output refresh epic validation passed"
updated_at: "2026-06-23T23:46:58.013671186+00:00"
---

## Summary

Human CLI output refresh epic validation passed

## Command

```console
sh -c 'cargo fmt -- --check && cargo check -p atelier-cli && cargo nextest run -p atelier-cli && target/debug/atelier lint atelier-c0qc && target/debug/atelier export --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 71
Truncated: no

```text
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 59463
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.83s
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.97s
────────────
 Nextest run ID 26ec1b51-ddaf-4560-af70-b55fd6a8cd09 with nextest profile: default
    Starting 476 tests across 4 binaries
        PASS [   0.010s] (  1/476) atelier-cli command_surface::tests::mission_status_verbose_reference_targets_subcommand_help
        PASS [   0.010s] (  2/476) atelier-cli command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.012s] (  3/476) atelier-cli command_surface::tests::expands_slash_command_references
        PASS [   0.013s] (  4/476) atelier-cli command_surface::tests::root_help_parser_includes_missions_section
        PASS [   0.025s] (  5/476) atelier-cli commands::create::tests::test_validate_priority_malicious
        PASS [   0.026s] (  6/476) atelier-cli commands::create::tests::test_get_template_exists
        PASS [   0.028s] (  7/476) atelier-cli commands::create::tests::test_template_bug_description_prefix
        PASS [   0.028s] (  8/476) atelier-cli commands::create::tests::test_validate_priority_valid
        PASS [   0.014s] (  9/476) atelier-cli commands::create::tests::test_template_fields
        PASS [   0.031s] ( 10/476) atelier-cli commands::comment::tests::test_validate_known_kinds
        PASS [   0.031s] ( 11/476) atelier-cli commands::comment::tests::test_validate_unknown_kinds
        PASS [   0.018s] ( 12/476) atelier-cli commands::create::tests::test_unknown_template_returns_none
        PASS [   0.025s] ( 13/476) atelier-cli commands::create::tests::test_validate_priority_invalid
        PASS [   0.026s] ( 14/476) atelier-cli commands::create::tests::test_list_templates
        PASS [   0.043s] ( 15/476) atelier-cli commands::create::tests::test_get_template_not_found
        PASS [   0.044s] ( 16/476) atelier-cli command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.044s] ( 17/476) atelier-cli commands::create::tests::test_template_feature_description_prefix
        PASS [   0.050s] ( 18/476) atelier-cli commands::create::tests::test_invalid_priorities_never_validate
        PASS [   0.017s] ( 19/476) atelier-cli commands::deps::tests::truncate_respects_limit
        PASS [   0.011s] ( 20/476) atelier-cli commands::forgejo::tests::inspect_roles_reports_success_and_collapsed_mappings
        PASS [   0.184s] ( 21/476) atelier-cli commands::comment::tests::test_add_comment_to_nonexistent_issue
        PASS [   0.188s] ( 22/476) atelier-cli commands::comment::tests::test_add_unicode_comment
        PASS [   0.011s] ( 23/476) atelier-cli commands::import::tests::test_imported_beads_description_uses_current_issue_sections
        PASS [   0.202s] ( 24/476) atelier-cli commands::comment::tests::test_add_multiple_comments
        PASS [   0.202s] ( 25/476) atelier-cli commands::comment::tests::test_comment_on_closed_issue
        PASS [   0.206s] ( 26/476) atelier-cli commands::comment::tests::test_add_comment_sql_injection
        PASS [   0.207s] ( 27/476) atelier-cli commands::comment::tests::test_add_comment_with_special_chars
        PASS [   0.207s] ( 28/476) atelier-cli commands::comment::tests::test_add_comment_with_newlines
        PASS [   0.216s] ( 29/476) atelier-cli commands::comment::tests::test_comment_with_null_bytes
        PASS [   0.204s] ( 30/476) atelier-cli commands::delete::tests::test_delete_closed_issue
        PASS [   0.208s] ( 31/476) atelier-cli commands::delete::tests::test_delete_cascades_comments
        PASS [   0.219s] ( 32/476) atelier-cli commands::comment::tests::test_add_comment_to_existing_issue
        PASS [   0.212s] ( 33/476) atelier-cli commands::delete::tests::test_delete_cascades_labels
        PASS [   0.222s] ( 34/476) atelier-cli commands::comment::tests::test_add_very_long_comment
        PASS [   0.224s] ( 35/4
```

