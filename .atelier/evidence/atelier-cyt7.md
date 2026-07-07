---
created_at: "2026-07-06T21:05:20.206593607+00:00"
id: "atelier-cyt7"
evidence_type: "test"
captured_at: "2026-07-06T21:04:22.068739467+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor b81cd933 HEAD\nscripts/check_active_command_guidance.sh --self-test\nscripts/check_active_command_guidance.sh --inventory\nscripts/check_active_command_guidance.sh\nscripts/check_active_quality_command_guidance.sh --self-test\nbash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh\nroot_help=\"$(target/debug/atelier --help)\"\nwork_help=\"$(target/debug/atelier work --help)\"\nissue_help=\"$(target/debug/atelier issue --help)\"\ncheck_help=\"$(target/debug/atelier check --help)\"\nfor current in work issue check; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$root_help\"; done\nfor removed in start worktree lint doctor maintenance provider recovery; do if rg -q \"^[[:space:]]+${removed}([[:space:]]|$)\" <<< \"$root_help\"; then exit 1; fi; done\nrg -q \"queue[[:space:]]+Show the legacy\" <<< \"$work_help\"\nfor current in ready blocked missions mission epic; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$work_help\"; done\nfor current in list show transition; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$issue_help\"; done\nrg -q -- \"--fix\" <<< \"$check_help\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ntarget/debug/atelier check\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\ntest -z \"$(git status --porcelain)\"\n'"
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
title: "atelier-z6q1 implementation proof (not independent validation): occurrence-aware Markdown/prose boundaries, command-local negation, 23 adversarial production-pipeline fixtures, full-index guard/inventory, wrapper parity, help parity, focused 9 tests, formatting, tracker health, and c0mp preservation pass"
updated_at: "2026-07-06T21:06:06.648283160+00:00"
---

## Summary

atelier-z6q1 implementation proof (not independent validation): occurrence-aware Markdown/prose boundaries, command-local negation, 23 adversarial production-pipeline fixtures, full-index guard/inventory, wrapper parity, help parity, focused 9 tests, formatting, tracker health, and c0mp preservation pass

## Command

```console
bash -lc '
set -euo pipefail
git merge-base --is-ancestor b81cd933 HEAD
scripts/check_active_command_guidance.sh --self-test
scripts/check_active_command_guidance.sh --inventory
scripts/check_active_command_guidance.sh
scripts/check_active_quality_command_guidance.sh --self-test
bash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh
root_help="$(target/debug/atelier --help)"
work_help="$(target/debug/atelier work --help)"
issue_help="$(target/debug/atelier issue --help)"
check_help="$(target/debug/atelier check --help)"
for current in work issue check; do rg -q "^[[:space:]]+${current}([[:space:]]|$)" <<< "$root_help"; done
for removed in start worktree lint doctor maintenance provider recovery; do if rg -q "^[[:space:]]+${removed}([[:space:]]|$)" <<< "$root_help"; then exit 1; fi; done
rg -q "queue[[:space:]]+Show the legacy" <<< "$work_help"
for current in ready blocked missions mission epic; do rg -q "^[[:space:]]+${current}([[:space:]]|$)" <<< "$work_help"; done
for current in list show transition; do rg -q "^[[:space:]]+${current}([[:space:]]|$)" <<< "$issue_help"; done
rg -q -- "--fix" <<< "$check_help"
cargo nextest run -p atelier-app command_surface
cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases
cargo fmt -- --check
git diff --check master...HEAD
target/debug/atelier check
git merge-base --is-ancestor 60634f2b HEAD
git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md
test -z "$(git status --porcelain)"
'
```

Exit status: 0

## Stdout

Bytes: 564
Truncated: no

```text
active command guidance self-test passed: 129 prohibited/context-restricted example(s), including 23 adversarial occurrence fixture(s) and all prior quality cases
command inventory passed: 11 visible, 10 hidden, 46 removed, 30 audit-token root(s), 23 indexed-guidance token root(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
active command guidance self-test passed: 129 prohibited/context-restricted example(s), including 23 adversarial occurrence fixture(s) and all prior quality cases
Lint passed.
```

## Stderr

Bytes: 1885
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
────────────
 Nextest run ID b7a99cc7-bcb0-4397-97f7-68baac1abd4a with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.009s] (2/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.009s] (3/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.009s] (4/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.009s] (5/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.009s] (6/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.010s] (7/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.015s] (8/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
────────────
     Summary [   0.016s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-durs-publish-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.20s
────────────
 Nextest run ID 591edb3f-efdc-4bc0-89a3-d5b1865caf4e with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.117s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.118s] 1 test run: 1 passed, 447 skipped
```
