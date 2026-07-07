---
created_at: "2026-07-06T21:28:20.813653414+00:00"
id: "atelier-nm5w"
evidence_type: "test"
captured_at: "2026-07-06T21:27:20.166719743+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor f53158a3 HEAD\nscripts/check_active_command_guidance.sh --self-test\nscripts/check_active_command_guidance.sh --inventory\nscripts/check_active_command_guidance.sh\nscripts/check_active_quality_command_guidance.sh --self-test\nbash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\nfixtures=(\n  \"Do not forget to run atelier start now.\"\n  \"You must not avoid atelier start.\"\n  \"Never skip atelier start.\"\n  \"atelier start must not be skipped; use it now.\"\n)\nfor fixture in \"${fixtures[@]}\"; do\n  output=$(printf \"# Live Guidance\\n%s\\n\" \"$fixture\" | active_content | scan_content)\n  test -n \"$output\"\n  output=$(printf \"## Historical Commands (Non-Normative)\\n%s\\n\" \"$fixture\" | active_content | scan_content)\n  test -z \"$output\"\n  output=$(printf \"## Historical Commands (Non-Normative)\\n%s\\n## Live Guidance\\n%s\\n\" \"$fixture\" \"$fixture\" | active_content | scan_content)\n  test -n \"$output\"\ndone\nprintf \"conservative production-pipeline batch passed: %d active rejections with section allowance/re-entry pairs\\n\" \"${#fixtures[@]}\"\nroot_help=\"$(target/debug/atelier --help)\"\nwork_help=\"$(target/debug/atelier work --help)\"\nissue_help=\"$(target/debug/atelier issue --help)\"\ncheck_help=\"$(target/debug/atelier check --help)\"\nfor current in work issue check; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$root_help\"; done\nfor removed in start worktree lint doctor maintenance provider recovery; do if rg -q \"^[[:space:]]+${removed}([[:space:]]|$)\" <<< \"$root_help\"; then exit 1; fi; done\nrg -q \"queue[[:space:]]+Show the legacy\" <<< \"$work_help\"\nfor current in ready blocked missions mission epic; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$work_help\"; done\nfor current in list show transition; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$issue_help\"; done\nrg -q -- \"--fix\" <<< \"$check_help\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ntarget/debug/atelier check\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\ntest -z \"$(git status --porcelain)\"\n'"
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
title: "atelier-ypgn implementation proof (not independent validation): strict Removed/Retired active-content rejection, exact four conservative fixtures with Historical allowance/live re-entry pairs, 139-case production-pipeline self-test, full scan/inventory, wrapper/help parity, focused 9 tests, formatting, tracker health, and c0mp preservation pass"
updated_at: "2026-07-06T21:28:51.493731523+00:00"
---

## Summary

atelier-ypgn implementation proof (not independent validation): strict Removed/Retired active-content rejection, exact four conservative fixtures with Historical allowance/live re-entry pairs, 139-case production-pipeline self-test, full scan/inventory, wrapper/help parity, focused 9 tests, formatting, tracker health, and c0mp preservation pass

## Command

```console
bash -lc '
set -euo pipefail
git merge-base --is-ancestor f53158a3 HEAD
scripts/check_active_command_guidance.sh --self-test
scripts/check_active_command_guidance.sh --inventory
scripts/check_active_command_guidance.sh
scripts/check_active_quality_command_guidance.sh --self-test
bash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
fixtures=(
  "Do not forget to run atelier start now."
  "You must not avoid atelier start."
  "Never skip atelier start."
  "atelier start must not be skipped; use it now."
)
for fixture in "${fixtures[@]}"; do
  output=$(printf "# Live Guidance\n%s\n" "$fixture" | active_content | scan_content)
  test -n "$output"
  output=$(printf "## Historical Commands (Non-Normative)\n%s\n" "$fixture" | active_content | scan_content)
  test -z "$output"
  output=$(printf "## Historical Commands (Non-Normative)\n%s\n## Live Guidance\n%s\n" "$fixture" "$fixture" | active_content | scan_content)
  test -n "$output"
done
printf "conservative production-pipeline batch passed: %d active rejections with section allowance/re-entry pairs\n" "${#fixtures[@]}"
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

Bytes: 669
Truncated: no

```text
active command guidance self-test passed: 139 prohibited/context-restricted example(s), including 33 adversarial occurrence fixture(s) and all prior quality cases
command inventory passed: 11 visible, 10 hidden, 46 removed, 28 audit-token root(s), 18 indexed-guidance token root(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
active command guidance self-test passed: 139 prohibited/context-restricted example(s), including 33 adversarial occurrence fixture(s) and all prior quality cases
conservative production-pipeline batch passed: 4 active rejections with section allowance/re-entry pairs
Lint passed.
```

## Stderr

Bytes: 1885
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID 3f25459f-e32d-477c-9b82-d91e1d343892 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.008s] (1/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.008s] (2/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.009s] (3/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.011s] (4/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.011s] (5/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.011s] (6/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.011s] (7/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.014s] (8/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
────────────
     Summary [   0.014s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-durs-publish-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.30s
────────────
 Nextest run ID 209607bf-89d5-4e43-80ad-68063fbfd43a with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.108s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.109s] 1 test run: 1 passed, 447 skipped
```
