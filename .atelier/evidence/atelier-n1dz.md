---
created_at: "2026-07-06T20:05:58.611239726+00:00"
id: "atelier-n1dz"
evidence_type: "test"
captured_at: "2026-07-06T20:05:48.042487120+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor f32d45d2 HEAD\nscripts/check_active_quality_command_guidance.sh --self-test\nscripts/check_active_quality_command_guidance.sh --inventory\nscripts/check_active_quality_command_guidance.sh\nbash -n scripts/check_active_quality_command_guidance.sh\nroot_help=\"$(target/debug/atelier --help)\"\nrg -q \"review\" <<< \"$root_help\"\nrg -q \"history\" <<< \"$root_help\"\nrg -q \"issue\" <<< \"$root_help\"\nfor retired in maintenance provider recovery revert supersede rework modify delete update worker orchestrator; do\n  if rg -q \"^[[:space:]]+${retired}([[:space:]]|$)\" <<< \"$root_help\"; then exit 1; fi\ndone\nreview_help=\"$(target/debug/atelier review --help)\"\nhistory_help=\"$(target/debug/atelier history --help)\"\nissue_help=\"$(target/debug/atelier issue --help)\"\nrg -q \"inspect|submit|resolve\" <<< \"$review_help\"\nrg -q \"issue\" <<< \"$history_help\"\nrg -q \"show|transition\" <<< \"$issue_help\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ntarget/debug/atelier check\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\n'"
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
title: "atelier-3ezp implementation proof (not independent validation): verdict-derived maintenance prohibition, coverage meta-test, broad categorized inventory, indexed quality scan, branch-built help parity, focused tests, formatting, branch diff, tracker health, and c0mp preservation pass"
updated_at: "2026-07-06T20:06:04.936817938+00:00"
---

## Summary

atelier-3ezp implementation proof (not independent validation): verdict-derived maintenance prohibition, coverage meta-test, broad categorized inventory, indexed quality scan, branch-built help parity, focused tests, formatting, branch diff, tracker health, and c0mp preservation pass

## Command

```console
bash -lc '
set -euo pipefail
git merge-base --is-ancestor f32d45d2 HEAD
scripts/check_active_quality_command_guidance.sh --self-test
scripts/check_active_quality_command_guidance.sh --inventory
scripts/check_active_quality_command_guidance.sh
bash -n scripts/check_active_quality_command_guidance.sh
root_help="$(target/debug/atelier --help)"
rg -q "review" <<< "$root_help"
rg -q "history" <<< "$root_help"
rg -q "issue" <<< "$root_help"
for retired in maintenance provider recovery revert supersede rework modify delete update worker orchestrator; do
  if rg -q "^[[:space:]]+${retired}([[:space:]]|$)" <<< "$root_help"; then exit 1; fi
done
review_help="$(target/debug/atelier review --help)"
history_help="$(target/debug/atelier history --help)"
issue_help="$(target/debug/atelier issue --help)"
rg -q "inspect|submit|resolve" <<< "$review_help"
rg -q "issue" <<< "$history_help"
rg -q "show|transition" <<< "$issue_help"
cargo nextest run -p atelier-app command_surface
cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases
cargo fmt -- --check
git diff --check master...HEAD
target/debug/atelier check
git merge-base --is-ancestor 60634f2b HEAD
git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md
'
```

Exit status: 0

## Stdout

Bytes: 245
Truncated: no

```text
active quality guidance self-test passed: 103 prohibited/context-restricted example(s)
command inventory passed: 11 visible, 10 hidden, 44 removed, 30 audit-token root(s)
active quality guidance check passed: 10 indexed document(s)
Lint passed.
```

## Stderr

Bytes: 1885
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
────────────
 Nextest run ID 208de743-68ef-4d8a-a814-169d2d4b872b with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.009s] (2/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.009s] (3/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.010s] (4/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.010s] (5/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.011s] (6/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.011s] (7/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.012s] (8/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
────────────
     Summary [   0.013s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-durs-publish-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.31s
────────────
 Nextest run ID 675d3fd1-3d51-439c-9526-b23b506418e3 with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.126s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.127s] 1 test run: 1 passed, 447 skipped
```
