---
created_at: "2026-07-06T19:54:17.997727361+00:00"
id: "atelier-z5rf"
evidence_type: "test"
captured_at: "2026-07-06T19:54:08.806088891+00:00"
command: "bash -lc 'set -euo pipefail; git merge-base --is-ancestor 516efd2e HEAD; scripts/check_active_quality_command_guidance.sh --self-test; scripts/check_active_quality_command_guidance.sh; bash -n scripts/check_active_quality_command_guidance.sh; root=$(target/debug/atelier --help); review=$(target/debug/atelier review --help); history=$(target/debug/atelier history --help); issue=$(target/debug/atelier issue --help); printf \"%s\\n\" \"$root\" | rg -q \"^  issue \"; printf \"%s\\n\" \"$root\" | rg -q \"^  review \"; printf \"%s\\n\" \"$root\" | rg -q \"^  history \"; for retired in mission graph plan start worktree repair note abandon search prime claim session dep lint doctor export rebuild import-beads maintenance; do ! printf \"%s\\n\" \"$root\" | rg -q \"^  ${retired} +\"; done; printf \"%s\\n\" \"$review\" | rg -q \"^  (open|show|merge|submit|resolve) +\"; for retired in link status comments comment approve request-changes; do ! printf \"%s\\n\" \"$review\" | rg -q \"^  ${retired} +\"; done; printf \"%s\\n\" \"$history\" | rg -q -- \"--issue <ISSUE>\"; printf \"%s\\n\" \"$history\" | rg -q -- \"--limit <LIMIT>\"; for retired in --mission --epic --include-descendants --event-kind --actor --since; do ! printf \"%s\\n\" \"$history\" | rg -q -- \"$retired\"; done; ! printf \"%s\\n\" \"$issue\" | rg -q \"^  close +\"; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; cargo fmt -- --check; git diff --check master...HEAD; target/debug/atelier check; git merge-base --is-ancestor 60634f2b HEAD; git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md; test -z \"$(git status --porcelain)\"; echo \"atelier-fc6x implementation remediation checks passed\"'"
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
title: "atelier-fc6x remediation: authoritative full-surface guard, deterministic negative self-test, indexed quality scan, branch-built help parity, focused tests, formatting, branch diff, tracker health, and c0mp preservation pass"
updated_at: "2026-07-06T19:54:24.191575517+00:00"
---

## Summary

atelier-fc6x remediation: authoritative full-surface guard, deterministic negative self-test, indexed quality scan, branch-built help parity, focused tests, formatting, branch diff, tracker health, and c0mp preservation pass

## Command

```console
bash -lc 'set -euo pipefail; git merge-base --is-ancestor 516efd2e HEAD; scripts/check_active_quality_command_guidance.sh --self-test; scripts/check_active_quality_command_guidance.sh; bash -n scripts/check_active_quality_command_guidance.sh; root=$(target/debug/atelier --help); review=$(target/debug/atelier review --help); history=$(target/debug/atelier history --help); issue=$(target/debug/atelier issue --help); printf "%s\n" "$root" | rg -q "^  issue "; printf "%s\n" "$root" | rg -q "^  review "; printf "%s\n" "$root" | rg -q "^  history "; for retired in mission graph plan start worktree repair note abandon search prime claim session dep lint doctor export rebuild import-beads maintenance; do ! printf "%s\n" "$root" | rg -q "^  ${retired} +"; done; printf "%s\n" "$review" | rg -q "^  (open|show|merge|submit|resolve) +"; for retired in link status comments comment approve request-changes; do ! printf "%s\n" "$review" | rg -q "^  ${retired} +"; done; printf "%s\n" "$history" | rg -q -- "--issue <ISSUE>"; printf "%s\n" "$history" | rg -q -- "--limit <LIMIT>"; for retired in --mission --epic --include-descendants --event-kind --actor --since; do ! printf "%s\n" "$history" | rg -q -- "$retired"; done; ! printf "%s\n" "$issue" | rg -q "^  close +"; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; cargo fmt -- --check; git diff --check master...HEAD; target/debug/atelier check; git merge-base --is-ancestor 60634f2b HEAD; git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md; test -z "$(git status --porcelain)"; echo "atelier-fc6x implementation remediation checks passed"'
```
Exit status: 0

## Stdout

Bytes: 214
Truncated: no

```text
active quality guidance self-test passed: 92 prohibited/context-restricted example(s)
active quality guidance check passed: 10 indexed document(s)
Lint passed.
atelier-fc6x implementation remediation checks passed
```

## Stderr

Bytes: 1885
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
────────────
 Nextest run ID c55d92ef-4c2b-46b5-8726-d3bced98b986 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.049s] (2/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.049s] (3/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.049s] (4/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.049s] (5/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.049s] (6/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.049s] (7/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.049s] (8/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
────────────
     Summary [   0.050s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-durs-publish-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.20s
────────────
 Nextest run ID c67ffd99-14ac-4e61-8dec-b390ee3af881 with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.096s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.097s] 1 test run: 1 passed, 447 skipped
```
