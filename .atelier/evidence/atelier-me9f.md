---
created_at: "2026-07-06T17:59:02.218903836+00:00"
id: "atelier-me9f"
evidence_type: "test"
captured_at: "2026-07-06T17:58:56.600464293+00:00"
command: "cargo nextest run -p atelier-app -p atelier-cli command_surface::tests::visible_grouped_review_references_target_subcommand_help command_surface::tests::subcommand_help_parser_extracts_commands_section review_help_exposes_only_the_collapsed_public_contract review_surface_derives_open_context_and_uses_submit_and_show smoke::lifecycle::test_dependency_chain_and_ready"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-8kv0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8kv0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Passed collapsed review help/behavior: show and submit replace status/comments/comment/approve/request-changes with no compatibility aliases."
updated_at: "2026-07-06T17:59:06.829480988+00:00"
---

## Summary

Passed collapsed review help/behavior: show and submit replace status/comments/comment/approve/request-changes with no compatibility aliases.

## Command

```console
cargo nextest run -p atelier-app -p atelier-cli command_surface::tests::visible_grouped_review_references_target_subcommand_help command_surface::tests::subcommand_help_parser_extracts_commands_section review_help_exposes_only_the_collapsed_public_contract review_surface_derives_open_context_and_uses_submit_and_show smoke::lifecycle::test_dependency_chain_and_ready
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1021
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.17s
────────────
 Nextest run ID 41273e4c-c9c8-4a7d-8423-eed034b15523 with nextest profile: default
    Starting 5 tests across 5 binaries (553 tests skipped)
        PASS [   0.010s] (1/5) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.011s] (2/5) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.057s] (3/5) atelier-cli::cli_integration review_help_exposes_only_the_collapsed_public_contract
        PASS [   0.954s] (4/5) atelier-cli::cli_integration review_surface_derives_open_context_and_uses_submit_and_show
        PASS [   3.139s] (5/5) atelier-cli::smoke_tests smoke::lifecycle::test_dependency_chain_and_ready
────────────
     Summary [   3.141s] 5 tests run: 5 passed, 553 skipped
```
