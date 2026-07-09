---
created_at: "2026-07-06T20:44:08.896539139+00:00"
id: "atelier-e3x2"
evidence_type: "validation"
captured_at: "2026-07-06T20:43:34.064763524+00:00"
command: "bash -lc 'set -euo pipefail; git merge-base --is-ancestor c4ec82f8 HEAD; scripts/check_active_command_guidance.sh --self-test; scripts/check_active_command_guidance.sh --inventory; scripts/check_active_command_guidance.sh; scripts/check_active_quality_command_guidance.sh; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; cargo fmt -- --check; git diff --check master...HEAD; git merge-base --is-ancestor 60634f2b HEAD; git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md; target/debug/atelier check; echo independent-full-index-validation-pass'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-durs"
    role: "validates"
  - kind: "issue"
    id: "atelier-p0am"
    role: "validates"
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "SUPPLEMENTAL PR51 REVALIDATION: PASS. Supersedes atelier-k19d in full, atelier-3ezp, and stale-guidance/guard-gap portions of atelier-fc6x and atelier-za9w. Also supersedes validator diagnostics atelier-gq7k, atelier-cwlj, atelier-fttx, atelier-kv6m, and atelier-tozp: the partial probes are non-proof; the comprehensive diagnostics passed their substantive checks and failed only because their self-referential clean-tree assertion observed evidence files created by the recorder. Independent meta-validation confirmed 88 indexed docs/10 quality docs, required-six and missing-index reachability, 11 visible/10 hidden/46 removed roots, 42 categorized raw roots including removed pr/migrate, 106 fixtures, historical/rejected exclusion and re-entry, compatibility parity, zero broad active findings, and zero exact prior-six normative findings. Attached transcript passes guards, focused 8+1 tests, fmt, master diff, c0mp preservation, and tracker health. C0mp implementation remains deferred/not-applicable to durs. No product/doc changes."
updated_at: "2026-07-06T20:45:32.024077531+00:00"
---

## Summary

SUPPLEMENTAL PR51 REVALIDATION: PASS. Supersedes atelier-k19d in full, atelier-3ezp, and stale-guidance/guard-gap portions of atelier-fc6x and atelier-za9w. Also supersedes validator diagnostics atelier-gq7k, atelier-cwlj, atelier-fttx, atelier-kv6m, and atelier-tozp: the partial probes are non-proof; the comprehensive diagnostics passed their substantive checks and failed only because their self-referential clean-tree assertion observed evidence files created by the recorder. Independent meta-validation confirmed 88 indexed docs/10 quality docs, required-six and missing-index reachability, 11 visible/10 hidden/46 removed roots, 42 categorized raw roots including removed pr/migrate, 106 fixtures, historical/rejected exclusion and re-entry, compatibility parity, zero broad active findings, and zero exact prior-six normative findings. Attached transcript passes guards, focused 8+1 tests, fmt, master diff, c0mp preservation, and tracker health. C0mp implementation remains deferred/not-applicable to durs. No product/doc changes.

## Command

```console
bash -lc 'set -euo pipefail; git merge-base --is-ancestor c4ec82f8 HEAD; scripts/check_active_command_guidance.sh --self-test; scripts/check_active_command_guidance.sh --inventory; scripts/check_active_command_guidance.sh; scripts/check_active_quality_command_guidance.sh; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; cargo fmt -- --check; git diff --check master...HEAD; git merge-base --is-ancestor 60634f2b HEAD; git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md; target/debug/atelier check; echo independent-full-index-validation-pass'
```

Exit status: 0

## Stdout

Bytes: 505
Truncated: no

```text
active command guidance self-test passed: 106 prohibited/context-restricted example(s), including all prior quality cases
command inventory passed: 11 visible, 10 hidden, 46 removed, 30 audit-token root(s), 23 indexed-guidance token root(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
Lint passed.
independent-full-index-validation-pass
```

## Stderr

Bytes: 1872
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
────────────
 Nextest run ID d3577de0-1337-4df8-a2e6-fa66740efc87 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.185s] (1/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.185s] (2/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.185s] (3/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.185s] (4/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.185s] (5/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.185s] (6/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.185s] (7/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.185s] (8/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
────────────
     Summary [   0.186s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.96s
────────────
 Nextest run ID 09e77734-c0dd-4505-a2c1-ecf02536a045 with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.114s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.115s] 1 test run: 1 passed, 447 skipped
```
