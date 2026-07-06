---
created_at: "2026-06-25T01:28:21.480189641+00:00"
id: "atelier-d6pp"
evidence_type: "test"
captured_at: "2026-06-25T01:28:19.115516161+00:00"
command: "cargo test -p atelier-cli command_surface::tests --lib -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-47cp"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-47cp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli command_surface::tests --lib -- --nocapture"
updated_at: "2026-06-25T01:28:29.522828828+00:00"
---

## Summary

cargo test -p atelier-cli command_surface::tests --lib -- --nocapture

## Command

```console
cargo test -p atelier-cli command_surface::tests --lib -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 823
Truncated: no

```text

running 8 tests
test command_surface::tests::nested_visible_group_references_target_nested_subcommand_help ... ok
test command_surface::tests::expands_slash_command_references ... ok
test command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections ... ok
test command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help ... ok
test command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent ... ok
test command_surface::tests::root_help_parser_includes_work_section ... ok
test command_surface::tests::subcommand_help_parser_extracts_commands_section ... ok
test command_surface::tests::visible_grouped_review_references_target_subcommand_help ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 148 filtered out; finished in 0.00s
```

## Stderr

Bytes: 432
Truncated: no

```text
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.28s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
```

