---
created_at: "2026-07-06T21:48:13.860930112+00:00"
id: "atelier-p2aq"
evidence_type: "test"
captured_at: "2026-07-06T21:47:12.521667025+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor f961e9a1 HEAD\nscripts/check_active_command_guidance.sh --self-test\nscripts/check_active_command_guidance.sh --inventory\nscripts/check_active_command_guidance.sh\nscripts/check_active_quality_command_guidance.sh --self-test\nbash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\ncheck_rejected() { output=$(printf \"%s\\n\" \"$1\" | active_content \"${2:-}\" | scan_content); test -n \"$output\"; }\ncheck_allowed() { output=$(printf \"%s\\n\" \"$1\" | active_content \"${2:-}\" | scan_content); test -z \"$output\"; }\ncheck_rejected $'\"'\"'## Legacy Queue Boundary Extended\\nUse `atelier work queue` now.'\"'\"' \"$repo_root/$legacy_callable_document\"\ncheck_rejected $'\"'\"'## Legacy Queue Boundary\\nUse `atelier work queue` now.'\"'\"' \"$repo_root/docs/product/work-view-ordering.md\"\ncheck_rejected $'\"'\"'# Live Guidance\\nThe hidden API is gone; use atelier doctor --fix now.'\"'\"'\ncheck_rejected $'\"'\"'# Live Guidance\\nThis is not a recovery command; use atelier export --check now.'\"'\"'\ncheck_rejected $'\"'\"'# Live Guidance\\nThe admin path differed; invoke atelier rebuild now.'\"'\"'\ncheck_rejected $'\"'\"'# Diagnostics Extended\\nUse atelier doctor --fix now.'\"'\"'\ncheck_allowed $'\"'\"'## Legacy Queue Boundary\\nThe legacy `atelier work queue` is bounded here.'\"'\"' \"$repo_root/$legacy_callable_document\"\ncheck_allowed $'\"'\"'# Diagnostics\\nUse atelier diagnostics slow.'\"'\"'\ncheck_allowed $'\"'\"'# Unrelated\\nUse atelier export --check now.'\"'\"' \"$repo_root/docs/product/command-audit/export.md\"\ncheck_rejected $'\"'\"'# Setup And Recovery\\nUse atelier doctor --fix.\\n# Live Guidance\\nUse atelier doctor --fix.'\"'\"'\nprintf \"exception-scope production-pipeline batch passed\\n\"\nroot_help=\"$(target/debug/atelier --help)\"\nwork_help=\"$(target/debug/atelier work --help)\"\nissue_help=\"$(target/debug/atelier issue --help)\"\ncheck_help=\"$(target/debug/atelier check --help)\"\nfor current in work issue check; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$root_help\"; done\nfor removed in start worktree lint doctor maintenance provider recovery; do if rg -q \"^[[:space:]]+${removed}([[:space:]]|$)\" <<< \"$root_help\"; then exit 1; fi; done\nrg -q \"queue[[:space:]]+Show the legacy\" <<< \"$work_help\"\nfor current in ready blocked missions mission epic; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$work_help\"; done\nfor current in list show transition; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$issue_help\"; done\nrg -q -- \"--fix\" <<< \"$check_help\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ntarget/debug/atelier check\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\ntest -z \"$(git status --porcelain)\"\n'"
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
title: "atelier-wnrb implementation proof (not independent validation): exact c0mp document/heading scope, section/document-scoped hidden-command contexts, mixed-context and spoof rejection, 144-case production-pipeline self-test, full scan/inventory, wrapper/help parity, focused 9 tests, formatting, tracker health, and c0mp preservation pass"
updated_at: "2026-07-06T21:48:44.756107379+00:00"
---

## Summary

atelier-wnrb implementation proof (not independent validation): exact c0mp document/heading scope, section/document-scoped hidden-command contexts, mixed-context and spoof rejection, 144-case production-pipeline self-test, full scan/inventory, wrapper/help parity, focused 9 tests, formatting, tracker health, and c0mp preservation pass

## Command

```console
bash -lc '
set -euo pipefail
git merge-base --is-ancestor f961e9a1 HEAD
scripts/check_active_command_guidance.sh --self-test
scripts/check_active_command_guidance.sh --inventory
scripts/check_active_command_guidance.sh
scripts/check_active_quality_command_guidance.sh --self-test
bash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
check_rejected() { output=$(printf "%s\n" "$1" | active_content "${2:-}" | scan_content); test -n "$output"; }
check_allowed() { output=$(printf "%s\n" "$1" | active_content "${2:-}" | scan_content); test -z "$output"; }
check_rejected $'"'"'## Legacy Queue Boundary Extended\nUse `atelier work queue` now.'"'"' "$repo_root/$legacy_callable_document"
check_rejected $'"'"'## Legacy Queue Boundary\nUse `atelier work queue` now.'"'"' "$repo_root/docs/product/work-view-ordering.md"
check_rejected $'"'"'# Live Guidance\nThe hidden API is gone; use atelier doctor --fix now.'"'"'
check_rejected $'"'"'# Live Guidance\nThis is not a recovery command; use atelier export --check now.'"'"'
check_rejected $'"'"'# Live Guidance\nThe admin path differed; invoke atelier rebuild now.'"'"'
check_rejected $'"'"'# Diagnostics Extended\nUse atelier doctor --fix now.'"'"'
check_allowed $'"'"'## Legacy Queue Boundary\nThe legacy `atelier work queue` is bounded here.'"'"' "$repo_root/$legacy_callable_document"
check_allowed $'"'"'# Diagnostics\nUse atelier diagnostics slow.'"'"'
check_allowed $'"'"'# Unrelated\nUse atelier export --check now.'"'"' "$repo_root/docs/product/command-audit/export.md"
check_rejected $'"'"'# Setup And Recovery\nUse atelier doctor --fix.\n# Live Guidance\nUse atelier doctor --fix.'"'"'
printf "exception-scope production-pipeline batch passed\n"
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

Bytes: 613
Truncated: no

```text
active command guidance self-test passed: 144 prohibited/context-restricted example(s), including 38 adversarial occurrence fixture(s) and all prior quality cases
command inventory passed: 11 visible, 10 hidden, 46 removed, 28 audit-token root(s), 18 indexed-guidance token root(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
active command guidance self-test passed: 144 prohibited/context-restricted example(s), including 38 adversarial occurrence fixture(s) and all prior quality cases
exception-scope production-pipeline batch passed
Lint passed.
```

## Stderr

Bytes: 1885
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID d2618940-cb04-4d91-abca-5035c0004ae2 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.009s] (2/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.010s] (3/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.010s] (4/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.010s] (5/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.011s] (6/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.012s] (7/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.016s] (8/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
────────────
     Summary [   0.017s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-durs-publish-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.19s
────────────
 Nextest run ID 7a5972e0-73ee-4aed-b4f9-5d5a631ab696 with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.104s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.105s] 1 test run: 1 passed, 447 skipped
```
