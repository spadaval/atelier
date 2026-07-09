---
created_at: "2026-07-06T19:28:51.205940245+00:00"
id: "atelier-17q6"
evidence_type: "test"
captured_at: "2026-07-06T19:28:44.669609092+00:00"
command: "bash -lc 'set -euo pipefail; git merge-base --is-ancestor 60634f2b HEAD; git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md; rg -Fq \"are separate read surfaces.\" docs/product/issue-inventory-and-mission-overview.md; rg -Fq \"is the **Mission Overview**\" docs/product/issue-inventory-and-mission-overview.md; rg -Fq \"remains the scoped drill-down.\" docs/product/issue-inventory-and-mission-overview.md; pattern=\"atelier (prime|start|issue list.*--(ready|blocked)|issue close|lint|doctor)\"; ! rg -n -i \"$pattern\" PRODUCT_INTENT.md CONTEXT.md; current=$(sed -n \"/^## Current Equivalent Workflow/,/^## Historical Failure Classifications/p\" docs/architecture/quality/agent-factory-atelier-validation.md); ! printf \"%s\\n\" \"$current\" | rg -n -i \"$pattern\"; rg -q \"^## Historical Evidence Summary \\\\(Non-Normative\\\\)$\" docs/architecture/quality/agent-factory-atelier-validation.md; root=$(target/debug/atelier --help); issue=$(target/debug/atelier issue --help); list=$(target/debug/atelier issue list --help); work=$(target/debug/atelier work --help); mission=$(target/debug/atelier work mission --help); check=$(target/debug/atelier check --help); printf \"%s\\n\" \"$root\" | rg -q \"^  work \"; printf \"%s\\n\" \"$root\" | rg -q \"^  issue \"; printf \"%s\\n\" \"$root\" | rg -q \"^  check \"; for retired in prime start doctor lint search session; do ! printf \"%s\\n\" \"$root\" | rg -q \"^  ${retired} +\"; done; printf \"%s\\n\" \"$issue\" | rg -q \"^  list +List issue records as generic inventory\"; ! printf \"%s\\n\" \"$issue\" | rg -q \"^  close +\"; printf \"%s\\n\" \"$list\" | rg -q -- \"--status <STATUS>\"; printf \"%s\\n\" \"$list\" | rg -q -- \"--category <CATEGORY>\"; printf \"%s\\n\" \"$work\" | rg -q \"^  missions +\"; printf \"%s\\n\" \"$work\" | rg -q \"^  mission +\"; printf \"%s\\n\" \"$mission\" | rg -q \"Usage: atelier work mission\"; printf \"%s\\n\" \"$check\" | rg -q -- \"--fix\"; cargo fmt -- --check; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; git diff --check master...HEAD; target/debug/atelier check; test -z \"$(git status --porcelain)\"; echo \"atelier-neip implementation remediation checks passed\"'"
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
title: "atelier-neip remediation: c0mp contract ancestry/content, normative guidance, branch-built help paths, focused tests, formatting, branch diff, and tracker health pass"
updated_at: "2026-07-06T19:28:59.090816404+00:00"
---

## Summary

atelier-neip remediation: c0mp contract ancestry/content, normative guidance, branch-built help paths, focused tests, formatting, branch diff, and tracker health pass

## Command

```console
bash -lc 'set -euo pipefail; git merge-base --is-ancestor 60634f2b HEAD; git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md; rg -Fq "are separate read surfaces." docs/product/issue-inventory-and-mission-overview.md; rg -Fq "is the **Mission Overview**" docs/product/issue-inventory-and-mission-overview.md; rg -Fq "remains the scoped drill-down." docs/product/issue-inventory-and-mission-overview.md; pattern="atelier (prime|start|issue list.*--(ready|blocked)|issue close|lint|doctor)"; ! rg -n -i "$pattern" PRODUCT_INTENT.md CONTEXT.md; current=$(sed -n "/^## Current Equivalent Workflow/,/^## Historical Failure Classifications/p" docs/architecture/quality/agent-factory-atelier-validation.md); ! printf "%s\n" "$current" | rg -n -i "$pattern"; rg -q "^## Historical Evidence Summary \\(Non-Normative\\)$" docs/architecture/quality/agent-factory-atelier-validation.md; root=$(target/debug/atelier --help); issue=$(target/debug/atelier issue --help); list=$(target/debug/atelier issue list --help); work=$(target/debug/atelier work --help); mission=$(target/debug/atelier work mission --help); check=$(target/debug/atelier check --help); printf "%s\n" "$root" | rg -q "^  work "; printf "%s\n" "$root" | rg -q "^  issue "; printf "%s\n" "$root" | rg -q "^  check "; for retired in prime start doctor lint search session; do ! printf "%s\n" "$root" | rg -q "^  ${retired} +"; done; printf "%s\n" "$issue" | rg -q "^  list +List issue records as generic inventory"; ! printf "%s\n" "$issue" | rg -q "^  close +"; printf "%s\n" "$list" | rg -q -- "--status <STATUS>"; printf "%s\n" "$list" | rg -q -- "--category <CATEGORY>"; printf "%s\n" "$work" | rg -q "^  missions +"; printf "%s\n" "$work" | rg -q "^  mission +"; printf "%s\n" "$mission" | rg -q "Usage: atelier work mission"; printf "%s\n" "$check" | rg -q -- "--fix"; cargo fmt -- --check; cargo nextest run -p atelier-app command_surface; cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases; git diff --check master...HEAD; target/debug/atelier check; test -z "$(git status --porcelain)"; echo "atelier-neip implementation remediation checks passed"'
```
Exit status: 0

## Stdout

Bytes: 67
Truncated: no

```text
Lint passed.
atelier-neip implementation remediation checks passed
```

## Stderr

Bytes: 1885
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
────────────
 Nextest run ID 06acb9a9-15c7-4736-a8a3-1d4aaa9177d0 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.010s] (2/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.010s] (3/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.011s] (4/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.011s] (5/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.011s] (6/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.013s] (7/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.014s] (8/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
────────────
     Summary [   0.015s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-durs-publish-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.45s
────────────
 Nextest run ID 0642c805-201c-4f33-89c4-11144a215670 with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.212s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.213s] 1 test run: 1 passed, 447 skipped
```
