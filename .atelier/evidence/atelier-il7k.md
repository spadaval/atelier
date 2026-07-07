---
created_at: "2026-07-06T20:29:48.871451959+00:00"
id: "atelier-il7k"
evidence_type: "test"
captured_at: "2026-07-06T20:29:13.656555755+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor 994b3507 HEAD\nscripts/check_active_command_guidance.sh --self-test\nscripts/check_active_command_guidance.sh --inventory\nscripts/check_active_command_guidance.sh\nscripts/check_active_quality_command_guidance.sh\nbash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh\nactive_normative() {\n  awk '\"'\"'\n    function level(line, marks) { marks=line; sub(/[^#].*$/, \"\", marks); return length(marks) }\n    /^#{1,6} / {\n      current=level($0)\n      if (excluded && current <= excluded_level) excluded=0\n      lower=tolower($0)\n      if (lower ~ /(historical|removed|retired).*(non-normative|commands|classification)/ || lower ~ /(rejected alternatives|alternatives considered)/) {\n        excluded=1\n        excluded_level=current\n      }\n    }\n    !excluded { print }\n  '\"'\"'\n}\nassert_absent() {\n  local pattern=$1\n  local file=$2\n  if active_normative < \"$file\" | rg -n -- \"$pattern\"; then exit 1; fi\n}\nassert_absent \"atelier work queue\" docs/product/work-view-ordering.md\nassert_absent \"Normal workflow.*work queue|work queue.*Normal workflow\" docs/product/command-audit/category-review.md\nassert_absent \"atelier work queue\" docs/architecture/markdown-first-record-store.md\nassert_absent \"atelier lint|atelier doctor --fix\" docs/spec/storage/export/rebuild/canonical-layout.md\nassert_absent \"atelier start|atelier issue close|atelier worktree for-mission\" docs/adr/0004-work-lock-sync-policy.md\nassert_absent \"atelier start|atelier issue close|atelier worktree\" docs/adr/0007-mission-workspaces-and-epic-review-branches.md\nprintf \"raw repository token roots:\\n\"\nrg -o --no-filename \"\\x60(target/debug/)?atelier [a-z0-9-]+\" CONTEXT.md PRODUCT_INTENT.md docs/product docs/architecture docs/spec docs/adr --glob \"*.md\" | sed -E \"s/^\\x60(target\\\\/debug\\\\/)?atelier //\" | sort -u\nroot_help=\"$(target/debug/atelier --help)\"\nwork_help=\"$(target/debug/atelier work --help)\"\nissue_help=\"$(target/debug/atelier issue --help)\"\ncheck_help=\"$(target/debug/atelier check --help)\"\nfor current in work issue check; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$root_help\"; done\nfor removed in start worktree lint doctor maintenance provider recovery; do if rg -q \"^[[:space:]]+${removed}([[:space:]]|$)\" <<< \"$root_help\"; then exit 1; fi; done\nrg -q \"queue[[:space:]]+Show the legacy\" <<< \"$work_help\"\nfor current in ready blocked missions mission epic; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$work_help\"; done\nfor current in list show transition; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$issue_help\"; done\nrg -q -- \"--fix\" <<< \"$check_help\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ntarget/debug/atelier check\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\ntest -z \"$(git status --porcelain)\"\n'"
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
title: "atelier-k19d implementation proof (not independent validation): six normative groups use current routing; historical rationale is explicitly non-normative; full-index guard, broad token inventory, help parity, focused tests, formatting, tracker health, and c0mp preservation pass"
updated_at: "2026-07-06T20:30:27.989688962+00:00"
---

## Summary

atelier-k19d implementation proof (not independent validation): six normative groups use current routing; historical rationale is explicitly non-normative; full-index guard, broad token inventory, help parity, focused tests, formatting, tracker health, and c0mp preservation pass

## Command

```console
bash -lc '
set -euo pipefail
git merge-base --is-ancestor 994b3507 HEAD
scripts/check_active_command_guidance.sh --self-test
scripts/check_active_command_guidance.sh --inventory
scripts/check_active_command_guidance.sh
scripts/check_active_quality_command_guidance.sh
bash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh
active_normative() {
  awk '"'"'
    function level(line, marks) { marks=line; sub(/[^#].*$/, "", marks); return length(marks) }
    /^#{1,6} / {
      current=level($0)
      if (excluded && current <= excluded_level) excluded=0
      lower=tolower($0)
      if (lower ~ /(historical|removed|retired).*(non-normative|commands|classification)/ || lower ~ /(rejected alternatives|alternatives considered)/) {
        excluded=1
        excluded_level=current
      }
    }
    !excluded { print }
  '"'"'
}
assert_absent() {
  local pattern=$1
  local file=$2
  if active_normative < "$file" | rg -n -- "$pattern"; then exit 1; fi
}
assert_absent "atelier work queue" docs/product/work-view-ordering.md
assert_absent "Normal workflow.*work queue|work queue.*Normal workflow" docs/product/command-audit/category-review.md
assert_absent "atelier work queue" docs/architecture/markdown-first-record-store.md
assert_absent "atelier lint|atelier doctor --fix" docs/spec/storage/export/rebuild/canonical-layout.md
assert_absent "atelier start|atelier issue close|atelier worktree for-mission" docs/adr/0004-work-lock-sync-policy.md
assert_absent "atelier start|atelier issue close|atelier worktree" docs/adr/0007-mission-workspaces-and-epic-review-branches.md
printf "raw repository token roots:\n"
rg -o --no-filename "\x60(target/debug/)?atelier [a-z0-9-]+" CONTEXT.md PRODUCT_INTENT.md docs/product docs/architecture docs/spec docs/adr --glob "*.md" | sed -E "s/^\x60(target\\/debug\\/)?atelier //" | sort -u
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

Bytes: 815
Truncated: no

```text
active command guidance self-test passed: 106 prohibited/context-restricted example(s), including all prior quality cases
command inventory passed: 11 visible, 10 hidden, 46 removed, 30 audit-token root(s), 23 indexed-guidance token root(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
raw repository token roots:
--help
abandon
archive
branch
bundle
check
close
current-work
diagnostics
doctor
evidence
export
finish
forgejo
graph
history
import-beads
init
issue
lint
maintenance
man
migrate
milestone
mission
note
orchestrator
plan
pr
prime
prune
rebuild
repair
review
search
session
start
status
timer
work
worker
workflow
worktree
Lint passed.
```

## Stderr

Bytes: 1885
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID a6e32cff-2c04-4f33-b4a0-710d3408dc92 with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.009s] (2/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.009s] (3/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.009s] (4/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.009s] (5/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.009s] (6/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.014s] (7/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.014s] (8/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
────────────
     Summary [   0.014s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-durs-publish-fix/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.42s
────────────
 Nextest run ID 102ff86f-b2e1-47ed-941c-dc87bc1014cb with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.098s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.100s] 1 test run: 1 passed, 447 skipped
```
