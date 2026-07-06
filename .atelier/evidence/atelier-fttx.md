---
created_at: "2026-07-06T20:41:13.557131935+00:00"
id: "atelier-fttx"
evidence_type: "validation"
captured_at: "2026-07-06T20:40:29.383436518+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor c4ec82f8 HEAD\nscripts/check_active_command_guidance.sh --self-test\nscripts/check_active_command_guidance.sh --inventory\nscripts/check_active_command_guidance.sh\nscripts/check_active_quality_command_guidance.sh\nbash -n scripts/check_active_command_guidance.sh scripts/check_active_quality_command_guidance.sh\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\ntest \"${#active_guidance_docs[@]}\" -eq 88\ntest \"${#quality_docs[@]}\" -eq 10\ntest \"${#missing_guidance_docs[@]}\" -eq 0\ntest \"${#visible_roots[@]}\" -eq 11\ntest \"${#hidden_index_roots[@]}\" -eq 10\ntest \"${#removed_roots[@]}\" -eq 46\nfor root in maintenance pr migrate; do array_contains \"$root\" \"${removed_roots[@]}\"; done\nfor rel in docs/product/work-view-ordering.md docs/product/command-audit/category-review.md docs/architecture/markdown-first-record-store.md docs/spec/storage/export/rebuild/canonical-layout.md docs/adr/0004-work-lock-sync-policy.md docs/adr/0007-mission-workspaces-and-epic-review-branches.md; do array_contains \"$repo_root/$rel\" \"${active_guidance_docs[@]}\"; done\nfor rel in \"${quality_docs[@]}\"; do array_contains \"$repo_root/docs/architecture/quality/$rel\" \"${active_guidance_docs[@]}\"; done\ntest -n \"$(missing_guidance_entries \"$repo_root/docs/product/__independent_missing__.md\")\"\ntest -n \"$(missing_quality_index_entries __independent_missing__.md)\"\ntest -z \"$(indexed_guidance_findings)\"\nactive_normative() {\n  awk '\"'\"'\n    function level(line, marks) { marks=line; sub(/[^#].*$/, \"\", marks); return length(marks) }\n    /^#{1,6} / {\n      current=level($0)\n      if (excluded && current <= excluded_level) excluded=0\n      lower=tolower($0)\n      if (lower ~ /(historical|removed|retired).*(non-normative|evidence|transcript|commands|behavior|surface|inventory|classification)/ || lower ~ /(rejected alternatives|alternatives considered)/) { excluded=1; excluded_level=current }\n    }\n    !excluded { print }\n  '\"'\"'\n}\nassert_absent() { local pattern=$1 file=$2; ! active_normative < \"$file\" | rg -n -- \"$pattern\"; }\nassert_absent \"atelier work queue\" docs/product/work-view-ordering.md\nassert_absent \"Normal workflow.*work queue|work queue.*Normal workflow\" docs/product/command-audit/category-review.md\nassert_absent \"atelier work queue\" docs/architecture/markdown-first-record-store.md\nassert_absent \"atelier lint|atelier doctor --fix\" docs/spec/storage/export/rebuild/canonical-layout.md\nassert_absent \"atelier start|atelier issue close|atelier worktree for-mission\" docs/adr/0004-work-lock-sync-policy.md\nassert_absent \"atelier start|atelier issue close|atelier worktree\" docs/adr/0007-mission-workspaces-and-epic-review-branches.md\nhistorical=\"$(printf \"# Historical Commands (Non-Normative)\\natelier start\\n## Nested History\\natelier lint\\n\" | active_content | scan_content)\"\ntest -z \"$historical\"\nreentry=\"$(printf \"# Historical Commands (Non-Normative)\\natelier start\\n# Current Guidance\\natelier start\\n\" | active_content | scan_content)\"\nrg -q \"atelier start\" <<< \"$reentry\"\nfor removed in pr migrate; do hit=\"$(printf \"# Live Guidance\\nUse \\x60atelier %s\\x60 now.\\n\" \"$removed\" | active_content | scan_content)\"; rg -q \"atelier $removed\" <<< \"$hit\"; done\nmapfile -t raw_roots < <(for path in \"${active_guidance_docs[@]}\"; do rg -o --no-filename \"\\x60(target/debug/)?atelier [a-z0-9-]+\" \"$path\" || true; done | sed -E \"s/^\\x60(target\\\\/debug\\\\/)?atelier //\" | rg -v \"^-\" | sort -u)\nfor root in \"${raw_roots[@]}\"; do array_contains \"$root\" \"${visible_roots[@]}\" || array_contains \"$root\" \"${hidden_index_roots[@]}\" || array_contains \"$root\" \"${removed_roots[@]}\" || [[ \"$root\" == help ]]; done\nroot_help=\"$(target/debug/atelier --help)\"\nreview_help=\"$(target/debug/atelier review --help)\"\nhistory_help=\"$(target/debug/atelier history --help)\"\nissue_help=\"$(target/debug/atelier issue --help)\"\nwork_help=\"$(target/debug/atelier work --help)\"\ncheck_help=\"$(target/debug/atelier check --help)\"\nfor current in issue review history work check; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$root_help\"; done\nfor removed in mission graph plan start worktree repair note abandon search maintenance provider recovery pr migrate lint doctor; do ! rg -q \"^[[:space:]]+${removed}([[:space:]]|$)\" <<< \"$root_help\"; done\nfor current in open show merge submit resolve; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$review_help\"; done\nrg -q -- \"--issue <ISSUE>\" <<< \"$history_help\"\nrg -q -- \"--limit <LIMIT>\" <<< \"$history_help\"\nfor current in list show transition; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$issue_help\"; done\nfor current in ready blocked missions mission epic; do rg -q \"^[[:space:]]+${current}([[:space:]]|$)\" <<< \"$work_help\"; done\nrg -q \"queue[[:space:]]+Show the legacy\" <<< \"$work_help\"\nrg -q -- \"--fix\" <<< \"$check_help\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\ntarget/debug/atelier check\ntest -z \"$(git status --porcelain)\"\necho \"independent full-index supplemental validation passed\"\n'"
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
title: "SUPPLEMENTAL PR51 REVALIDATION: PASS. Supersedes atelier-k19d in full, atelier-3ezp, and the stale-guidance/guard-gap portions of atelier-fc6x and atelier-za9w; prior independent mission behavior classifications remain unchanged. The full-index guard reaches 88 repository-indexed documents including all 10 quality docs and the six prior failure sources, detects missing indexed paths, derives 11 visible/10 hidden/46 removed command roots authoritatively, classifies all raw indexed command tokens including removed pr/migrate, passes 106 prohibited/context-restricted fixtures, preserves historical/rejected-section exclusion with same-level re-entry, and keeps the old quality entrypoint behavior-identical. Independent broad active-guidance scan and exact prior-six scan have zero normative findings. Branch help, focused tests 8+1, fmt, master-range whitespace, c0mp ancestry/exact content, tracker health, and clean-state checks pass. C0mp implementation remains deferred/not-applicable to durs under the existing mission boundary. No validator product/doc changes made."
updated_at: "2026-07-06T20:41:20.084248249+00:00"
---

## Summary

SUPPLEMENTAL PR51 REVALIDATION: PASS. Supersedes atelier-k19d in full, atelier-3ezp, and the stale-guidance/guard-gap portions of atelier-fc6x and atelier-za9w; prior independent mission behavior classifications remain unchanged. The full-index guard reaches 88 repository-indexed documents including all 10 quality docs and the six prior failure sources, detects missing indexed paths, derives 11 visible/10 hidden/46 removed command roots authoritatively, classifies all raw indexed command tokens including removed pr/migrate, passes 106 prohibited/context-restricted fixtures, preserves historical/rejected-section exclusion with same-level re-entry, and keeps the old quality entrypoint behavior-identical. Independent broad active-guidance scan and exact prior-six scan have zero normative findings. Branch help, focused tests 8+1, fmt, master-range whitespace, c0mp ancestry/exact content, tracker health, and clean-state checks pass. C0mp implementation remains deferred/not-applicable to durs under the existing mission boundary. No validator product/doc changes made.

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
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
test "${#active_guidance_docs[@]}" -eq 88
test "${#quality_docs[@]}" -eq 10
test "${#missing_guidance_docs[@]}" -eq 0
test "${#visible_roots[@]}" -eq 11
test "${#hidden_index_roots[@]}" -eq 10
test "${#removed_roots[@]}" -eq 46
for root in maintenance pr migrate; do array_contains "$root" "${removed_roots[@]}"; done
for rel in docs/product/work-view-ordering.md docs/product/command-audit/category-review.md docs/architecture/markdown-first-record-store.md docs/spec/storage/export/rebuild/canonical-layout.md docs/adr/0004-work-lock-sync-policy.md docs/adr/0007-mission-workspaces-and-epic-review-branches.md; do array_contains "$repo_root/$rel" "${active_guidance_docs[@]}"; done
for rel in "${quality_docs[@]}"; do array_contains "$repo_root/docs/architecture/quality/$rel" "${active_guidance_docs[@]}"; done
test -n "$(missing_guidance_entries "$repo_root/docs/product/__independent_missing__.md")"
test -n "$(missing_quality_index_entries __independent_missing__.md)"
test -z "$(indexed_guidance_findings)"
active_normative() {
  awk '"'"'
    function level(line, marks) { marks=line; sub(/[^#].*$/, "", marks); return length(marks) }
    /^#{1,6} / {
      current=level($0)
      if (excluded && current <= excluded_level) excluded=0
      lower=tolower($0)
      if (lower ~ /(historical|removed|retired).*(non-normative|evidence|transcript|commands|behavior|surface|inventory|classification)/ || lower ~ /(rejected alternatives|alternatives considered)/) { excluded=1; excluded_level=current }
    }
    !excluded { print }
  '"'"'
}
assert_absent() { local pattern=$1 file=$2; ! active_normative < "$file" | rg -n -- "$pattern"; }
assert_absent "atelier work queue" docs/product/work-view-ordering.md
assert_absent "Normal workflow.*work queue|work queue.*Normal workflow" docs/product/command-audit/category-review.md
assert_absent "atelier work queue" docs/architecture/markdown-first-record-store.md
assert_absent "atelier lint|atelier doctor --fix" docs/spec/storage/export/rebuild/canonical-layout.md
assert_absent "atelier start|atelier issue close|atelier worktree for-mission" docs/adr/0004-work-lock-sync-policy.md
assert_absent "atelier start|atelier issue close|atelier worktree" docs/adr/0007-mission-workspaces-and-epic-review-branches.md
historical="$(printf "# Historical Commands (Non-Normative)\natelier start\n## Nested History\natelier lint\n" | active_content | scan_content)"
test -z "$historical"
reentry="$(printf "# Historical Commands (Non-Normative)\natelier start\n# Current Guidance\natelier start\n" | active_content | scan_content)"
rg -q "atelier start" <<< "$reentry"
for removed in pr migrate; do hit="$(printf "# Live Guidance\nUse \x60atelier %s\x60 now.\n" "$removed" | active_content | scan_content)"; rg -q "atelier $removed" <<< "$hit"; done
mapfile -t raw_roots < <(for path in "${active_guidance_docs[@]}"; do rg -o --no-filename "\x60(target/debug/)?atelier [a-z0-9-]+" "$path" || true; done | sed -E "s/^\x60(target\\/debug\\/)?atelier //" | rg -v "^-" | sort -u)
for root in "${raw_roots[@]}"; do array_contains "$root" "${visible_roots[@]}" || array_contains "$root" "${hidden_index_roots[@]}" || array_contains "$root" "${removed_roots[@]}" || [[ "$root" == help ]]; done
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

Bytes: 1872
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
────────────
 Nextest run ID c72da899-e395-4915-80ec-0701de826eab with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.009s] (2/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.009s] (3/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.009s] (4/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.009s] (5/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.009s] (6/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.009s] (7/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.010s] (8/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
────────────
     Summary [   0.011s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.24s
────────────
 Nextest run ID 80693103-c2a5-4587-a229-c34d5b9966c8 with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.102s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.103s] 1 test run: 1 passed, 447 skipped
```
