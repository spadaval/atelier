---
created_at: "2026-07-07T00:04:34.036338873+00:00"
id: "atelier-1x4g"
evidence_type: "validation"
captured_at: "2026-07-07T00:01:35.769729220+00:00"
command: "bash -lc '\nset -euo pipefail\ntest \"$(git rev-parse HEAD)\" = 2cad1dc033ac80c23af05f11d88ae21ded2e73b4\nfor id in atelier-ztnc atelier-igno atelier-xs4r atelier-ypbt atelier-sa0l; do\n  cmd=$(perl -0777 -ne \"if (/## Command\\n\\n\\x60\\x60\\x60console\\n(.*?)\\n\\x60\\x60\\x60/s) { print \\$1 }\" \".atelier/evidence/$id.md\")\n  eval \"$cmd\"\n  printf \"%s exact matrix: PASS\\n\" \"$id\"\ndone\nprefix=\"$(sed \"/^if \\[\\[ \\${1:-} ==/,\\$d\" scripts/check_active_command_guidance.sh | sed \"s|^repo_root=.*|repo_root=$(pwd)|\")\"\neval \"$prefix\"\nfailures=0\nchecked=0\nhit() { local label=$1 input=$2 output; checked=$((checked+1)); output=$(printf \"%b\\n\" \"$input\" | active_content | scan_content); if [[ -z \"$output\" ]]; then printf \"MISS [%s]\\n%b\\n\" \"$label\" \"$input\"; failures=$((failures+1)); fi; }\nclear() { local label=$1 input=$2 output; checked=$((checked+1)); output=$(printf \"%b\\n\" \"$input\" | active_content | scan_content); if [[ -n \"$output\" ]]; then printf \"FALSE POSITIVE [%s]\\n%b\\n=> %s\\n\" \"$label\" \"$input\" \"$output\"; failures=$((failures+1)); fi; }\nhit long-migration \"# Live\\nMove from the inherited old local operator workflow command with additional recovery context to \\`mission show atelier-demo\\`.\"\nhit migration-same-table-cell \"# Live\\n| Replacement | Move from the inherited old local operator workflow command to \\`mission show atelier-demo\\`. |\"\nclear migration-cannot-cross-table-pipe \"# Live\\n| Move from the inherited old workflow to | descriptor only | \\`mission\\` as the record type |\"\nclear passive-data \"# Live\\nUse \\`mission\\` to continue to be used as the record type.\"\nclear representation-data \"# Live\\nUse \\`mission\\` to represent the record type.\"\nhit actionable-show \"# Live\\nUse \\`mission\\` to show the transition status.\"\nhit actionable-inspect \"# Live\\nUse \\`mission\\` to inspect the transition status.\"\nhit repeated-environments-final-marker \"# Live\\n\\`\\`\\`console\\n(py) (git:main) user@host\\$ lint --all\\n\\`\\`\\`\"\nhit host-path-final-marker \"# Live\\n\\`\\`\\`console\\nuser@host ~/repo \\$ doctor --fix\\n\\`\\`\\`\"\nhit hash-final-marker \"# Live\\n\\`\\`\\`console\\n(env-a) (env-b) user@host:/repo # doctor --fix\\n\\`\\`\\`\"\nclear literal-dollar \"# Live\\n\\`\\`\\`console\\ncost: \\$5\\n\\`\\`\\`\"\nclear literal-percent \"# Live\\n\\`\\`\\`console\\nprogress: 100% complete\\n\\`\\`\\`\"\nclear literal-hash \"# Live\\n\\`\\`\\`console\\nvalue # comment\\n\\`\\`\\`\"\nclear immediate-indented-relation \"# Live\\n\\`\\`\\`\\nmission atelier-demo\\n  advances epic atelier-child\\n\\`\\`\\`\"\nhit blank-breaks-adjacency \"# Live\\n\\`\\`\\`\\nmission atelier-demo\\n\\n  advances epic atelier-child\\n\\`\\`\\`\"\nhit nonindented-is-not-relation \"# Live\\n\\`\\`\\`\\nmission atelier-demo\\nadvances epic atelier-child\\n\\`\\`\\`\"\nprintf \"fixed boundary matrix: %d cases, %d failures\\n\" \"$checked\" \"$failures\"\n((failures == 0))\nscripts/check_active_command_guidance.sh --self-test\nscripts/check_active_command_guidance.sh --inventory\nscripts/check_active_command_guidance.sh\nscripts/check_active_quality_command_guidance.sh --self-test\nscripts/check_active_quality_command_guidance.sh --inventory\nscripts/check_active_quality_command_guidance.sh\nmapfile -t findings < <(indexed_guidance_findings)\nprintf \"independent broad scan: active=%d quality=%d findings=%d\\n\" \"${#active_guidance_docs[@]}\" \"${#quality_docs[@]}\" \"${#findings[@]}\"\n((${#active_guidance_docs[@]} == 88 && ${#quality_docs[@]} == 10 && ${#findings[@]} == 0))\nroot=$(target/debug/atelier --help)\nfor current in issue review history work check; do grep -Eq \"^  ${current}[[:space:]]\" <<<\"$root\"; done\nfor removed in mission graph plan start worktree repair note abandon search maintenance provider recovery pr migrate lint doctor; do ! grep -Eq \"^  ${removed}[[:space:]]\" <<<\"$root\"; done\nreview=$(target/debug/atelier review --help)\nfor sub in open show merge submit resolve; do grep -Eq \"^  ${sub}[[:space:]]\" <<<\"$review\"; done\nhistory=$(target/debug/atelier history --help); grep -q -- \"--issue\" <<<\"$history\"; grep -q -- \"--limit\" <<<\"$history\"\nissue=$(target/debug/atelier issue --help)\nfor sub in list show transition; do grep -Eq \"^  ${sub}[[:space:]]\" <<<\"$issue\"; done\nwork=$(target/debug/atelier work --help)\nfor sub in ready blocked missions mission epic; do grep -Eq \"^  ${sub}[[:space:]]\" <<<\"$work\"; done\ngrep -Eq \"^  queue[[:space:]]+Show the legacy repo-wide operational queue\" <<<\"$work\"\ncheck=$(target/debug/atelier check --help); grep -q -- \"--fix\" <<<\"$check\"\nprintf \"help contract: PASS\\n\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\nprintf \"c0mp ancestry and exact contract: PASS\\n\"\ntarget/debug/atelier check\n'"
exit_status: "0"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-durs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS independent mission validation at exact requested head 2cad1dc0, code 77b16fb9. Supersedes atelier-sa0l. Exact prior 45 cases and the five atelier-sa0l repros pass; a fixed 16-case boundary matrix passes for unbounded same-line migration and table pipes, semantic descriptor versus actionable suffixes, final prompt markers and literal-symbol negatives, and graph adjacency. All 373 self-tests, production and quality wrappers, inventory 11 visible/10 hidden/46 removed, live 88 docs/10 quality/0 findings, help, focused Rust 9 of 9, fmt, master diff, c0mp 60634f2b ancestry and exact contract, and tracker check pass."
updated_at: "2026-07-07T00:05:06.121057964+00:00"
---

## Summary

PASS independent mission validation at exact requested head 2cad1dc0, code 77b16fb9. Supersedes atelier-sa0l. Exact prior 45 cases and the five atelier-sa0l repros pass; a fixed 16-case boundary matrix passes for unbounded same-line migration and table pipes, semantic descriptor versus actionable suffixes, final prompt markers and literal-symbol negatives, and graph adjacency. All 373 self-tests, production and quality wrappers, inventory 11 visible/10 hidden/46 removed, live 88 docs/10 quality/0 findings, help, focused Rust 9 of 9, fmt, master diff, c0mp 60634f2b ancestry and exact contract, and tracker check pass.

## Command

```console
bash -lc '
set -euo pipefail
test "$(git rev-parse HEAD)" = 2cad1dc033ac80c23af05f11d88ae21ded2e73b4
for id in atelier-ztnc atelier-igno atelier-xs4r atelier-ypbt atelier-sa0l; do
  cmd=$(perl -0777 -ne "if (/## Command\n\n\x60\x60\x60console\n(.*?)\n\x60\x60\x60/s) { print \$1 }" ".atelier/evidence/$id.md")
  eval "$cmd"
  printf "%s exact matrix: PASS\n" "$id"
done
prefix="$(sed "/^if \[\[ \${1:-} ==/,\$d" scripts/check_active_command_guidance.sh | sed "s|^repo_root=.*|repo_root=$(pwd)|")"
eval "$prefix"
failures=0
checked=0
hit() { local label=$1 input=$2 output; checked=$((checked+1)); output=$(printf "%b\n" "$input" | active_content | scan_content); if [[ -z "$output" ]]; then printf "MISS [%s]\n%b\n" "$label" "$input"; failures=$((failures+1)); fi; }
clear() { local label=$1 input=$2 output; checked=$((checked+1)); output=$(printf "%b\n" "$input" | active_content | scan_content); if [[ -n "$output" ]]; then printf "FALSE POSITIVE [%s]\n%b\n=> %s\n" "$label" "$input" "$output"; failures=$((failures+1)); fi; }
hit long-migration "# Live\nMove from the inherited old local operator workflow command with additional recovery context to \`mission show atelier-demo\`."
hit migration-same-table-cell "# Live\n| Replacement | Move from the inherited old local operator workflow command to \`mission show atelier-demo\`. |"
clear migration-cannot-cross-table-pipe "# Live\n| Move from the inherited old workflow to | descriptor only | \`mission\` as the record type |"
clear passive-data "# Live\nUse \`mission\` to continue to be used as the record type."
clear representation-data "# Live\nUse \`mission\` to represent the record type."
hit actionable-show "# Live\nUse \`mission\` to show the transition status."
hit actionable-inspect "# Live\nUse \`mission\` to inspect the transition status."
hit repeated-environments-final-marker "# Live\n\`\`\`console\n(py) (git:main) user@host\$ lint --all\n\`\`\`"
hit host-path-final-marker "# Live\n\`\`\`console\nuser@host ~/repo \$ doctor --fix\n\`\`\`"
hit hash-final-marker "# Live\n\`\`\`console\n(env-a) (env-b) user@host:/repo # doctor --fix\n\`\`\`"
clear literal-dollar "# Live\n\`\`\`console\ncost: \$5\n\`\`\`"
clear literal-percent "# Live\n\`\`\`console\nprogress: 100% complete\n\`\`\`"
clear literal-hash "# Live\n\`\`\`console\nvalue # comment\n\`\`\`"
clear immediate-indented-relation "# Live\n\`\`\`\nmission atelier-demo\n  advances epic atelier-child\n\`\`\`"
hit blank-breaks-adjacency "# Live\n\`\`\`\nmission atelier-demo\n\n  advances epic atelier-child\n\`\`\`"
hit nonindented-is-not-relation "# Live\n\`\`\`\nmission atelier-demo\nadvances epic atelier-child\n\`\`\`"
printf "fixed boundary matrix: %d cases, %d failures\n" "$checked" "$failures"
((failures == 0))
scripts/check_active_command_guidance.sh --self-test
scripts/check_active_command_guidance.sh --inventory
scripts/check_active_command_guidance.sh
scripts/check_active_quality_command_guidance.sh --self-test
scripts/check_active_quality_command_guidance.sh --inventory
scripts/check_active_quality_command_guidance.sh
mapfile -t findings < <(indexed_guidance_findings)
printf "independent broad scan: active=%d quality=%d findings=%d\n" "${#active_guidance_docs[@]}" "${#quality_docs[@]}" "${#findings[@]}"
((${#active_guidance_docs[@]} == 88 && ${#quality_docs[@]} == 10 && ${#findings[@]} == 0))
root=$(target/debug/atelier --help)
for current in issue review history work check; do grep -Eq "^  ${current}[[:space:]]" <<<"$root"; done
for removed in mission graph plan start worktree repair note abandon search maintenance provider recovery pr migrate lint doctor; do ! grep -Eq "^  ${removed}[[:space:]]" <<<"$root"; done
review=$(target/debug/atelier review --help)
for sub in open show merge submit resolve; do grep -Eq "^  ${sub}[[:space:]]" <<<"$review"; done
history=$(target/debug/atelier history --help); grep -q -- "--issue" <<<"$history"; grep -q -- "--limit" <<<"$history"
issue=$(target/debug/atelier issue --help)
for sub in list show transition; do grep -Eq "^  ${sub}[[:space:]]" <<<"$issue"; done
work=$(target/debug/atelier work --help)
for sub in ready blocked missions mission epic; do grep -Eq "^  ${sub}[[:space:]]" <<<"$work"; done
grep -Eq "^  queue[[:space:]]+Show the legacy repo-wide operational queue" <<<"$work"
check=$(target/debug/atelier check --help); grep -q -- "--fix" <<<"$check"
printf "help contract: PASS\n"
cargo nextest run -p atelier-app command_surface
cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases
cargo fmt -- --check
git diff --check master...HEAD
git merge-base --is-ancestor 60634f2b HEAD
git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md
printf "c0mp ancestry and exact contract: PASS\n"
target/debug/atelier check
'
```

Exit status: 0

## Stdout

Bytes: 1108
Truncated: no

```text
atelier-ztnc exact matrix: PASS
atelier-igno exact matrix: PASS
atelier-xs4r exact matrix: PASS
atelier-ypbt exact matrix: PASS
atelier-sa0l exact matrix: PASS
fixed boundary matrix: 16 cases, 0 failures
active command guidance self-test passed: 373 prohibited/context-restricted example(s), including 38 adversarial occurrence fixture(s) and all prior quality cases
command inventory passed: 11 visible, 10 hidden, 46 removed, 28 audit-token root(s), 18 indexed-guidance token root(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
active command guidance self-test passed: 373 prohibited/context-restricted example(s), including 38 adversarial occurrence fixture(s) and all prior quality cases
command inventory passed: 11 visible, 10 hidden, 46 removed, 28 audit-token root(s), 18 indexed-guidance token root(s)
active command guidance check passed: 88 repository-indexed document(s), including 10 quality document(s)
independent broad scan: active=88 quality=10 findings=0
help contract: PASS
c0mp ancestry and exact contract: PASS
Lint passed.
```

## Stderr

Bytes: 2076
Truncated: no

```text
bare structural boundary failures: 0
systematic structural failures: 0
bounded extension failures: 0
bounded category-invariant failures: 0
declared-grammar boundary failures: 0
Broken pipe (os error 32)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID 8ed184a6-3a5e-4cb4-836b-6e817020754d with nextest profile: default
    Starting 8 tests across 1 binary (98 tests skipped)
        PASS [   0.009s] (1/8) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.009s] (2/8) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.009s] (3/8) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.010s] (4/8) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.010s] (5/8) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.010s] (6/8) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.011s] (7/8) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.012s] (8/8) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
────────────
     Summary [   0.012s] 8 tests run: 8 passed, 98 skipped
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.96s
────────────
 Nextest run ID e00daced-adfc-44fa-b3e6-dadda4691141 with nextest profile: default
    Starting 1 test across 4 binaries (447 tests skipped)
        PASS [   0.106s] (1/1) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.107s] 1 test run: 1 passed, 447 skipped
```
