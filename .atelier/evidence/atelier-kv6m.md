---
created_at: "2026-07-06T20:42:29.724266947+00:00"
id: "atelier-kv6m"
evidence_type: "validation"
captured_at: "2026-07-06T20:41:54.574922962+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor c4ec82f8 HEAD\nscripts/check_active_command_guidance.sh --self-test\nscripts/check_active_command_guidance.sh --inventory\nscripts/check_active_command_guidance.sh\nscripts/check_active_quality_command_guidance.sh\nbash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh\nroot_help=\"$(target/debug/atelier --help)\"\nreview_help=\"$(target/debug/atelier review --help)\"\nhistory_help=\"$(target/debug/atelier history --help)\"\nissue_help=\"$(target/debug/atelier issue --help)\"\nwork_help=\"$(target/debug/atelier work --help)\"\ncheck_help=\"$(target/debug/atelier check --help)\"\nfor current in issue review history work check; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$root_help\"; done\nfor removed in mission graph plan start worktree repair note abandon search maintenance provider recovery pr migrate lint doctor; do ! rg -q \"^[[:space:]]+${removed}([[:space:]]|$)\" <<< \"$root_help\"; done\nfor current in open show merge submit resolve; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$review_help\"; done\nrg -q -- \"--issue <ISSUE>\" <<< \"$history_help\"\nrg -q -- \"--limit <LIMIT>\" <<< \"$history_help\"\nfor current in list show transition; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$issue_help\"; done\nfor current in ready blocked missions mission epic; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$work_help\"; done\nrg -q \"queue[[:space:]]+Show the legacy\" <<< \"$work_help\"\nrg -q -- \"--fix\" <<< \"$check_help\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\ntarget/debug/atelier check\ntest -z \"$(git status --porcelain)\"\necho \"independent full-index supplemental validation passed\"\n'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "SUPPLEMENTAL PR51 REVALIDATION: PASS. Supersedes atelier-k19d in full, atelier-3ezp, and the stale-guidance/guard-gap portions of atelier-fc6x and atelier-za9w; diagnostic probe atelier-gq7k is non-proof and superseded. Independent meta-validation confirmed recursive coverage of 88 repository-indexed docs including all 10 quality docs and the prior six sources; missing-index, required-six, and quality reachability checks; authoritative 11 visible/10 hidden/46 removed derivation; all 42 raw indexed roots categorized including removed pr/migrate; 106 fixtures; historical/rejected exclusion, nested exclusion, and same-level re-entry; compatibility entrypoint parity; zero broad active-guidance findings; and zero findings in the exact prior six normative groups. The attached transcript reruns guard modes, help parity, focused tests 8+1, fmt, master-range whitespace, c0mp ancestry/exact content, tracker health, and clean state. C0mp implementation remains deferred/not-applicable to durs. No validator product/doc changes made."
updated_at: "2026-07-06T20:42:36.160203153+00:00"
---

## Summary

SUPPLEMENTAL PR51 REVALIDATION: PASS. Supersedes atelier-k19d in full, atelier-3ezp, and the stale-guidance/guard-gap portions of atelier-fc6x and atelier-za9w; diagnostic probe atelier-gq7k is non-proof and superseded. Independent meta-validation confirmed recursive coverage of 88 repository-indexed docs including all 10 quality docs and the prior six sources; missing-index, required-six, and quality reachability checks; authoritative 11 visible/10 hidden/46 removed derivation; all 42 raw indexed roots categorized including removed pr/migrate; 106 fixtures; historical/rejected exclusion, nested exclusion, and same-level re-entry; compatibility entrypoint parity; zero broad active-guidance findings; and zero findings in the exact prior six normative groups. The attached transcript reruns guard modes, help parity, focused tests 8+1, fmt, master-range whitespace, c0mp ancestry/exact content, tracker health, and clean state. C0mp implementation remains deferred/not-applicable to durs. No validator product/doc changes made.

## Command

```console
bash -lc '
set -euo pipefail
git merge-base --is-ancestor c4ec82f8 HEAD
scripts/check_active_command_guidance.sh --self-test
scripts/check_active_command_guidance.sh --inventory
scripts/check_active_command_guidance.sh
scripts/check_active_quality_command_guidance.sh
bash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh
root_help="$(target/debug/atelier --help)"
review_help="$(target/debug/atelier review --help)"
history_help="$(target/debug/atelier history --help)"
issue_help="$(target/debug/atelier issue --help)"
work_help="$(target/debug/atelier work --help)"
check_help="$(target/debug/atelier check --help)"
for current in issue review history work check; do rg -q "^[[:space:]]+${current}([[:space:]]|$)" <<< "$root_help"; done
for removed in mission graph plan start worktree repair note abandon search maintenance provider recovery pr migrate lint doctor; do ! rg -q "^[[:space:]]+${removed}([[:space:]]|$)" <<< "$root_help"; done
for current in open show merge submit resolve; do rg -q "^[[:space:]]+${current}([[:space:]]|$)" <<< "$review_help"; done
rg -q -- "--issue <ISSUE>" <<< "$history_help"
rg -q -- "--limit <LIMIT>" <<< "$history_help"
for current in list show transition; do rg -q "^[[:space:]]+${current}([[:space:]]|$)" <<< "$issue_help"; done
for current in ready blocked missions mission epic; do rg -q "^[[:space:]]+${current}([[:space:]]|$)" <<< "$work_help"; done
rg -q "queue[[:space:]]+Show the legacy" <<< "$work_help"
rg -q -- "--fix" <<< "$check_help"
cargo nextest run -p atelier-app command_surface
cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases
cargo fmt -- --check
git diff --check master...HEAD
git merge-base --is-ancestor 60634f2b HEAD
git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md
target/debug/atelier check
test -z "$(git status --porcelain)"
echo "independent full-index supplemental validation passed"
'
```

Exit status: 1

## Stdout

Bytes: 466
Truncated: no

```text
active command guidance self-test passed: 106 prohibited/context-restricted example(s), including all prior quality cases
command inventory passed: 11 visible, 10 hidden, 46 removed, 30 audit-token root(s), 23 indexed-guidance token root(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
Lint passed.
```

## Stderr

Bytes: 1898
Truncated: no

```text
Broken pipe (os error 32)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID ba9f2d94-4a2d-43f1-874d-91e391929b55 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.009s] (2/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.010s] (3/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.011s] (4/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.011s] (5/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.012s] (6/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.012s] (7/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.013s] (8/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
────────────
     Summary [   0.014s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
────────────
 Nextest run ID e76d943c-06f9-4250-bb8a-976cea110e8c with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.097s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.098s] 1 test run: 1 passed, 447 skipped
```
