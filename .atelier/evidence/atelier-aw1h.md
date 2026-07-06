---
created_at: "2026-07-06T18:52:56.992568674+00:00"
id: "atelier-aw1h"
evidence_type: "test"
captured_at: "2026-07-06T18:52:52.515733850+00:00"
command: "bash -lc 'set -euo pipefail; atelier --help | rg -q \"issue +Create, list, show, update, transition, note, and manage links\"; atelier issue --help | rg -q \"transition +Show or execute issue transitions\"; atelier work --help | rg -q \"missions +List mission records by issue_type\"; atelier work --help | rg -q \"blocked +Show blocked work\"; atelier check --help | rg -q -- \"--fix\"; atelier init --help | rg -q -- \"--import-beads\"; atelier evidence --help | rg -q \"record +Record proof\"; cargo fmt -- --check; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; atelier check; git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-p0am"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p0am"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "atelier-vqhi remediation: current help ownership, command-surface tests, formatting, tracker health, and whitespace checks pass"
updated_at: "2026-07-06T18:53:00.887784036+00:00"
---

## Summary

atelier-vqhi remediation: current help ownership, command-surface tests, formatting, tracker health, and whitespace checks pass

## Command

```console
bash -lc 'set -euo pipefail; atelier --help | rg -q "issue +Create, list, show, update, transition, note, and manage links"; atelier issue --help | rg -q "transition +Show or execute issue transitions"; atelier work --help | rg -q "missions +List mission records by issue_type"; atelier work --help | rg -q "blocked +Show blocked work"; atelier check --help | rg -q -- "--fix"; atelier init --help | rg -q -- "--import-beads"; atelier evidence --help | rg -q "record +Record proof"; cargo fmt -- --check; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; atelier check; git diff --check'
```

Exit status: 0

## Stdout

Bytes: 13
Truncated: no

```text
Lint passed.
```

## Stderr

Bytes: 1877
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID e092b759-403e-41b7-8fd8-d928cb725285 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.009s] (2/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.009s] (3/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.009s] (4/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.011s] (5/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.012s] (6/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.020s] (7/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.020s] (8/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
────────────
     Summary [   0.022s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p0am-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.17s
────────────
 Nextest run ID 8828a80b-eac4-46d4-b794-0f11f8d9a3bd with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.103s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.104s] 1 test run: 1 passed, 447 skipped
```

