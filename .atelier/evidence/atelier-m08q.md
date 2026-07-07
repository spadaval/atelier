---
created_at: "2026-07-06T19:38:10.938276962+00:00"
id: "atelier-m08q"
evidence_type: "test"
captured_at: "2026-07-06T19:38:04.945529321+00:00"
command: "bash -lc 'set -euo pipefail; git merge-base --is-ancestor b2069dda HEAD; scripts/check_active_quality_command_guidance.sh; bash -n scripts/check_active_quality_command_guidance.sh; root=$(target/debug/atelier --help); issue=$(target/debug/atelier issue --help); work=$(target/debug/atelier work --help); check=$(target/debug/atelier check --help); printf \"%s\\n\" \"$root\" | rg -q \"^  issue \"; printf \"%s\\n\" \"$root\" | rg -q \"^  work \"; printf \"%s\\n\" \"$root\" | rg -q \"^  check \"; for retired in prime start doctor lint search session; do ! printf \"%s\\n\" \"$root\" | rg -q \"^  ${retired} +\"; done; printf \"%s\\n\" \"$issue\" | rg -q \"^  transition +\"; ! printf \"%s\\n\" \"$issue\" | rg -q \"^  close +\"; printf \"%s\\n\" \"$work\" | rg -q \"^  missions +\"; printf \"%s\\n\" \"$work\" | rg -q \"^  mission +\"; printf \"%s\\n\" \"$check\" | rg -q -- \"--fix\"; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; cargo fmt -- --check; git diff --check master...HEAD; target/debug/atelier check; test -z \"$(git status --porcelain)\"; echo \"atelier-68lu implementation remediation checks passed\"'"
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
title: "atelier-68lu remediation: indexed active-quality guidance scan, branch-built help paths, focused command-surface tests, formatting, branch diff, and tracker health pass"
updated_at: "2026-07-06T19:38:17.167530428+00:00"
---

## Summary

atelier-68lu remediation: indexed active-quality guidance scan, branch-built help paths, focused command-surface tests, formatting, branch diff, and tracker health pass

## Command

```console
bash -lc 'set -euo pipefail; git merge-base --is-ancestor b2069dda HEAD; scripts/check_active_quality_command_guidance.sh; bash -n scripts/check_active_quality_command_guidance.sh; root=$(target/debug/atelier --help); issue=$(target/debug/atelier issue --help); work=$(target/debug/atelier work --help); check=$(target/debug/atelier check --help); printf "%s\n" "$root" | rg -q "^  issue "; printf "%s\n" "$root" | rg -q "^  work "; printf "%s\n" "$root" | rg -q "^  check "; for retired in prime start doctor lint search session; do ! printf "%s\n" "$root" | rg -q "^  ${retired} +"; done; printf "%s\n" "$issue" | rg -q "^  transition +"; ! printf "%s\n" "$issue" | rg -q "^  close +"; printf "%s\n" "$work" | rg -q "^  missions +"; printf "%s\n" "$work" | rg -q "^  mission +"; printf "%s\n" "$check" | rg -q -- "--fix"; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; cargo fmt -- --check; git diff --check master...HEAD; target/debug/atelier check; test -z "$(git status --porcelain)"; echo "atelier-68lu implementation remediation checks passed"'
```
Exit status: 0

## Stdout

Bytes: 128
Truncated: no

```text
active quality guidance check passed: 10 indexed document(s)
Lint passed.
atelier-68lu implementation remediation checks passed
```

## Stderr

Bytes: 1885
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
────────────
 Nextest run ID b264c68f-87d7-45ce-9ff5-eb68bb657bd3 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.010s] (2/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.009s] (3/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.010s] (4/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.010s] (5/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.011s] (6/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.011s] (7/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.012s] (8/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
────────────
     Summary [   0.013s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-durs-publish-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.28s
────────────
 Nextest run ID 081faae6-125d-4b93-8dc1-6e28a5dbb4a4 with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.107s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.108s] 1 test run: 1 passed, 447 skipped
```
