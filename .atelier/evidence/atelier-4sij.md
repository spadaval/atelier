---
created_at: "2026-06-25T01:05:14.625268068+00:00"
id: "atelier-4sij"
evidence_type: "test"
captured_at: "2026-06-25T01:05:13.185776405+00:00"
command: "cargo test -p atelier-cli command_surface::tests --lib"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3uew"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3uew"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli command_surface::tests --lib"
updated_at: "2026-06-25T01:05:19.263376254+00:00"
---

## Summary

cargo test -p atelier-cli command_surface::tests --lib

## Command

```console
cargo test -p atelier-cli command_surface::tests --lib
```

Exit status: 0

## Stdout

Bytes: 821
Truncated: no

```text

running 8 tests
test command_surface::tests::expands_slash_command_references ... ok
test command_surface::tests::nested_visible_group_references_target_nested_subcommand_help ... ok
test command_surface::tests::mission_status_verbose_reference_targets_subcommand_help ... ok
test command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent ... ok
test command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections ... ok
test command_surface::tests::root_help_parser_includes_missions_section ... ok
test command_surface::tests::subcommand_help_parser_extracts_commands_section ... ok
test command_surface::tests::visible_grouped_review_references_target_subcommand_help ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 148 filtered out; finished in 0.00s
```

## Stderr

Bytes: 219
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.37s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
```

