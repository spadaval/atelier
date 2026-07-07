---
created_at: "2026-07-07T00:59:29.829465368+00:00"
id: "atelier-5t9p"
evidence_type: "validation"
captured_at: "2026-07-07T00:56:49.813749407+00:00"
command: "bash -lc '\nset -euo pipefail\ntest \"$(git rev-parse HEAD)\" = 1ba6a4e49f70b003866e7edbe5a33d3cbf056c6b\ntest \"$(git rev-parse HEAD^1)\" = ba9e7cfd2e237a040306c4ea08aea62800cbbdde\ntest \"$(git rev-parse HEAD^2)\" = 2578e236bb3970c292af0a0501f72f6c95f84f8b\nfor id in atelier-ztnc atelier-igno atelier-xs4r atelier-ypbt atelier-sa0l; do\n  cmd=$(perl -0777 -ne \"if (/## Command\\n\\n\\x60\\x60\\x60console\\n(.*?)\\n\\x60\\x60\\x60/s) { print \\$1 }\" \".atelier/evidence/$id.md\")\n  eval \"$cmd\"\n  printf \"%s exact matrix: PASS\\n\" \"$id\"\ndone\nprefix=\"$(sed \"/^if \\[\\[ \\${1:-} ==/,\\$d\" scripts/check_active_command_guidance.sh | sed \"s|^repo_root=.*|repo_root=$(pwd)|\")\"\neval \"$prefix\"\nfailures=0; checked=0\nhit() { local label=$1 input=$2 output; checked=$((checked+1)); output=$(printf \"%b\\n\" \"$input\" | active_content | scan_content); if [[ -z \"$output\" ]]; then printf \"MISS [%s]\\n%b\\n\" \"$label\" \"$input\"; failures=$((failures+1)); fi; }\nclear() { local label=$1 input=$2 output; checked=$((checked+1)); output=$(printf \"%b\\n\" \"$input\" | active_content | scan_content); if [[ -n \"$output\" ]]; then printf \"FALSE POSITIVE [%s]\\n%b\\n=> %s\\n\" \"$label\" \"$input\" \"$output\"; failures=$((failures+1)); fi; }\nhit long-migration \"# Live\\nMove from the inherited old local operator workflow command with additional recovery context to \\`mission show atelier-demo\\`.\"\nhit migration-same-table-cell \"# Live\\n| Replacement | Move from the inherited old local operator workflow command to \\`mission show atelier-demo\\`. |\"\nclear migration-cannot-cross-table-pipe \"# Live\\n| Move from the inherited old workflow to | descriptor only | \\`mission\\` as the record type |\"\nclear passive-data \"# Live\\nUse \\`mission\\` to continue to be used as the record type.\"\nclear representation-data \"# Live\\nUse \\`mission\\` to represent the record type.\"\nhit actionable-show \"# Live\\nUse \\`mission\\` to show the transition status.\"\nhit actionable-inspect \"# Live\\nUse \\`mission\\` to inspect the transition status.\"\nhit repeated-environments-final-marker \"# Live\\n\\`\\`\\`console\\n(py) (git:main) user@host\\$ lint --all\\n\\`\\`\\`\"\nhit host-path-final-marker \"# Live\\n\\`\\`\\`console\\nuser@host ~/repo \\$ doctor --fix\\n\\`\\`\\`\"\nhit hash-final-marker \"# Live\\n\\`\\`\\`console\\n(env-a) (env-b) user@host:/repo # doctor --fix\\n\\`\\`\\`\"\nclear literal-dollar \"# Live\\n\\`\\`\\`console\\ncost: \\$5\\n\\`\\`\\`\"\nclear literal-percent \"# Live\\n\\`\\`\\`console\\nprogress: 100% complete\\n\\`\\`\\`\"\nclear literal-hash \"# Live\\n\\`\\`\\`console\\nvalue # comment\\n\\`\\`\\`\"\nclear immediate-indented-relation \"# Live\\n\\`\\`\\`\\nmission atelier-demo\\n  advances epic atelier-child\\n\\`\\`\\`\"\nhit blank-breaks-adjacency \"# Live\\n\\`\\`\\`\\nmission atelier-demo\\n\\n  advances epic atelier-child\\n\\`\\`\\`\"\nhit nonindented-is-not-relation \"# Live\\n\\`\\`\\`\\nmission atelier-demo\\nadvances epic atelier-child\\n\\`\\`\\`\"\nprintf \"fixed boundary matrix: %d cases, %d failures\\n\" \"$checked\" \"$failures\"\n((failures == 0))\nscripts/check_active_command_guidance.sh --self-test\nscripts/check_active_command_guidance.sh --inventory\nscripts/check_active_command_guidance.sh\nscripts/check_active_quality_command_guidance.sh --self-test\nscripts/check_active_quality_command_guidance.sh --inventory\nscripts/check_active_quality_command_guidance.sh\nroot=$(target/debug/atelier --help)\nfor current in init man status work issue bundle evidence review history check prune; do grep -Eq \"^  ${current}[[:space:]]\" <<<\"$root\"; done\nfor hidden in forgejo branch rebuild workflow diagnostics export import-beads doctor lint; do ! grep -Eq \"^  ${hidden}[[:space:]]\" <<<\"$root\"; target/debug/atelier \"$hidden\" --help >/dev/null; done\nfor removed in mission graph plan start worktree repair note abandon search maintenance provider recovery pr migrate; do ! grep -Eq \"^  ${removed}[[:space:]]\" <<<\"$root\"; done\nreview=$(target/debug/atelier review --help)\nfor sub in open show merge submit resolve; do grep -Eq \"^  ${sub}[[:space:]]\" <<<\"$review\"; done\nfor removed in status link comments comment approve request-changes; do ! target/debug/atelier review \"$removed\" --help >/dev/null 2>&1; done\nopen=$(target/debug/atelier review open --help); grep -q -- \"--existing\" <<<\"$open\"; ! grep -q -- \"--title\" <<<\"$open\"; ! grep -q -- \"--source-branch\" <<<\"$open\"; ! grep -q -- \"--target-branch\" <<<\"$open\"\nhistory=$(target/debug/atelier history --help); grep -q -- \"--issue\" <<<\"$history\"; grep -q -- \"--limit\" <<<\"$history\"\nfor removed in --mission --epic --include-descendants --event-kind --actor --since; do ! target/debug/atelier history \"$removed\" x >/dev/null 2>&1; done\ncheck=$(target/debug/atelier check --help); grep -q -- \"--fix\" <<<\"$check\"\n! target/debug/atelier maintenance delete issue atelier-demo --force >/dev/null 2>&1\nprintf \"integration help/hidden/removed matrix: PASS\\n\"\n! git grep -n \"<<<<<<< \"\n! git grep -n \">>>>>>> \"\ntest ! -e crates/atelier-cli/src/commands/delete.rs\n! rg -n \"MaintenanceCommands|commands::delete|ReviewCommands::Status|status_outcome_resolves_linked_pr_without_rendering\" crates/atelier-cli/src crates/atelier-app/src/pr.rs\nrg -q \"RecordStore::new\\(repo_root.join\\(\\\".atelier\\\"\\)\\)\\.load_issue_by_id\" crates/atelier-app/src/pr.rs\ntest \"$(rg -c \"^### Diagnostics$\" docs/architecture/quality/validation.md)\" -eq 1\nbase=docs/spec/storage/export/rebuild\nfor path in ../../../../architecture/markdown-first-record-store.md ../../../../architecture/sqlite-runtime-schema.md ../../../../adr/0017-sqlite-domain-cache-and-hard-removal.md; do test -f \"$base/$path\"; done\nprintf \"static conflict-resolution assertions: PASS\\n\"\nhome=$(mktemp -d /tmp/atelier-durs-evidence-home.XXXXXX)\ntrap \"rm -rf \\\"$home\\\"\" EXIT\nHOME=\"$home\" CARGO_HOME=\"${CARGO_HOME:-/root/.cargo}\" RUSTUP_HOME=\"${RUSTUP_HOME:-/root/.rustup}\" cargo nextest run\nset +e\nHOME=\"$home\" CARGO_HOME=\"${CARGO_HOME:-/root/.cargo}\" RUSTUP_HOME=\"${RUSTUP_HOME:-/root/.rustup}\" cargo nextest run --profile extended --run-ignored=only\nextended=$?\nset -e\nprintf \"extended ignored exit=%d\\n\" \"$extended\"\ntest \"$extended\" -eq 4\ncargo fmt -- --check\ngit diff --check\ngit diff --check origin/master...HEAD\ntest \"$(git rev-parse origin/master)\" = 2578e236bb3970c292af0a0501f72f6c95f84f8b\ngit merge-base --is-ancestor HEAD^1 HEAD\ngit merge-base --is-ancestor origin/master HEAD\nmerged_tree=$(git merge-tree --write-tree HEAD origin/master)\ntest \"$merged_tree\" = \"$(git rev-parse HEAD^{tree})\"\nprintf \"diff/parents/ancestry/mergeability: PASS\\n\"\n'"
exit_status: "0"
agent_identity: "independent-integration-validator"
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
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS independent post-integration validation at exact two-parent merge 1ba6a4e4 with parents ba9e7cfd and master 2578e236, superseding pre-integration atelier-1x4g. All 16 conflict resolutions preserve master record-file and lazy domain-cache behavior plus durs review open/show/submit, bounded repo/issue history, quiet evidence, visible check repair, hidden provider/recovery, maintenance-delete removal, and no aliases. RecordStore review descriptions, obsolete review-status API/test removal, three authorized link corrections, diagnostics heading, and delete.rs deletion verified. Prior exact 50 and fixed 16 matrix pass; scanner 373, wrappers 88/10, focused 25 tests, isolated-HOME full 658/658, help, fmt, diff, ancestry/mergeability, and branch-built check pass. Extended ignored profile has zero tests with expected no-tests exit 4."
updated_at: "2026-07-07T00:59:47.621403873+00:00"
---

## Summary

PASS independent post-integration validation at exact two-parent merge 1ba6a4e4 with parents ba9e7cfd and master 2578e236, superseding pre-integration atelier-1x4g. All 16 conflict resolutions preserve master record-file and lazy domain-cache behavior plus durs review open/show/submit, bounded repo/issue history, quiet evidence, visible check repair, hidden provider/recovery, maintenance-delete removal, and no aliases. RecordStore review descriptions, obsolete review-status API/test removal, three authorized link corrections, diagnostics heading, and delete.rs deletion verified. Prior exact 50 and fixed 16 matrix pass; scanner 373, wrappers 88/10, focused 25 tests, isolated-HOME full 658/658, help, fmt, diff, ancestry/mergeability, and branch-built check pass. Extended ignored profile has zero tests with expected no-tests exit 4.

## Command

```console
bash -lc '
set -euo pipefail
test "$(git rev-parse HEAD)" = 1ba6a4e49f70b003866e7edbe5a33d3cbf056c6b
test "$(git rev-parse HEAD^1)" = ba9e7cfd2e237a040306c4ea08aea62800cbbdde
test "$(git rev-parse HEAD^2)" = 2578e236bb3970c292af0a0501f72f6c95f84f8b
for id in atelier-ztnc atelier-igno atelier-xs4r atelier-ypbt atelier-sa0l; do
  cmd=$(perl -0777 -ne "if (/## Command\n\n\x60\x60\x60console\n(.*?)\n\x60\x60\x60/s) { print \$1 }" ".atelier/evidence/$id.md")
  eval "$cmd"
  printf "%s exact matrix: PASS\n" "$id"
done
prefix="$(sed "/^if \[\[ \${1:-} ==/,\$d" scripts/check_active_command_guidance.sh | sed "s|^repo_root=.*|repo_root=$(pwd)|")"
eval "$prefix"
failures=0; checked=0
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
root=$(target/debug/atelier --help)
for current in init man status work issue bundle evidence review history check prune; do grep -Eq "^  ${current}[[:space:]]" <<<"$root"; done
for hidden in forgejo branch rebuild workflow diagnostics export import-beads doctor lint; do ! grep -Eq "^  ${hidden}[[:space:]]" <<<"$root"; target/debug/atelier "$hidden" --help >/dev/null; done
for removed in mission graph plan start worktree repair note abandon search maintenance provider recovery pr migrate; do ! grep -Eq "^  ${removed}[[:space:]]" <<<"$root"; done
review=$(target/debug/atelier review --help)
for sub in open show merge submit resolve; do grep -Eq "^  ${sub}[[:space:]]" <<<"$review"; done
for removed in status link comments comment approve request-changes; do ! target/debug/atelier review "$removed" --help >/dev/null 2>&1; done
open=$(target/debug/atelier review open --help); grep -q -- "--existing" <<<"$open"; ! grep -q -- "--title" <<<"$open"; ! grep -q -- "--source-branch" <<<"$open"; ! grep -q -- "--target-branch" <<<"$open"
history=$(target/debug/atelier history --help); grep -q -- "--issue" <<<"$history"; grep -q -- "--limit" <<<"$history"
for removed in --mission --epic --include-descendants --event-kind --actor --since; do ! target/debug/atelier history "$removed" x >/dev/null 2>&1; done
check=$(target/debug/atelier check --help); grep -q -- "--fix" <<<"$check"
! target/debug/atelier maintenance delete issue atelier-demo --force >/dev/null 2>&1
printf "integration help/hidden/removed matrix: PASS\n"
! git grep -n "<<<<<<< "
! git grep -n ">>>>>>> "
test ! -e crates/atelier-cli/src/commands/delete.rs
! rg -n "MaintenanceCommands|commands::delete|ReviewCommands::Status|status_outcome_resolves_linked_pr_without_rendering" crates/atelier-cli/src crates/atelier-app/src/pr.rs
rg -q "RecordStore::new\(repo_root.join\(\".atelier\"\)\)\.load_issue_by_id" crates/atelier-app/src/pr.rs
test "$(rg -c "^### Diagnostics$" docs/architecture/quality/validation.md)" -eq 1
base=docs/spec/storage/export/rebuild
for path in ../../../../architecture/markdown-first-record-store.md ../../../../architecture/sqlite-runtime-schema.md ../../../../adr/0017-sqlite-domain-cache-and-hard-removal.md; do test -f "$base/$path"; done
printf "static conflict-resolution assertions: PASS\n"
home=$(mktemp -d /tmp/atelier-durs-evidence-home.XXXXXX)
trap "rm -rf \"$home\"" EXIT
HOME="$home" CARGO_HOME="${CARGO_HOME:-/root/.cargo}" RUSTUP_HOME="${RUSTUP_HOME:-/root/.rustup}" cargo nextest run
set +e
HOME="$home" CARGO_HOME="${CARGO_HOME:-/root/.cargo}" RUSTUP_HOME="${RUSTUP_HOME:-/root/.rustup}" cargo nextest run --profile extended --run-ignored=only
extended=$?
set -e
printf "extended ignored exit=%d\n" "$extended"
test "$extended" -eq 4
cargo fmt -- --check
git diff --check
git diff --check origin/master...HEAD
test "$(git rev-parse origin/master)" = 2578e236bb3970c292af0a0501f72f6c95f84f8b
git merge-base --is-ancestor HEAD^1 HEAD
git merge-base --is-ancestor origin/master HEAD
merged_tree=$(git merge-tree --write-tree HEAD origin/master)
test "$merged_tree" = "$(git rev-parse HEAD^{tree})"
printf "diff/parents/ancestry/mergeability: PASS\n"
'
```

Exit status: 0

## Stdout

Bytes: 1134
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
integration help/hidden/removed matrix: PASS
static conflict-resolution assertions: PASS
extended ignored exit=4
diff/parents/ancestry/mergeability: PASS
```

## Stderr

Bytes: 79337
Truncated: yes

```text
bare structural boundary failures: 0
systematic structural failures: 0
bounded extension failures: 0
bounded category-invariant failures: 0
declared-grammar boundary failures: 0
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.92s
────────────
 Nextest run ID 6bd07555-8168-484e-91d5-8f09a3926ac6 with nextest profile: default
    Starting 658 tests across 9 binaries
        PASS [   0.009s] (  1/658) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.009s] (  2/658) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.011s] (  3/658) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.012s] (  4/658) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.012s] (  5/658) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.015s] (  6/658) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.023s] (  7/658) atelier-app forgejo::tests::provider_contract_accepts_only_official_approved_event
        PASS [   0.024s] (  8/658) atelier-app forgejo::tests::finds_and_shows_pull_request_state
        PASS [   0.024s] (  9/658) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.025s] ( 10/658) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.025s] ( 11/658) atelier-app health::tests::review_backend_health_skips_room_mode
        PASS [   0.025s] ( 12/658) atelier-app forgejo::tests::merges_pull_with_role_sudo_header_and_confirms_state
        PASS [   0.025s] ( 13/658) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.025s] ( 14/658) atelier-app health::tests::review_backend_health_reports_missing_provider_token_without_secret
        PASS [   0.025s] ( 15/658) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
        PASS [   0.026s] ( 16/658) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.027s] ( 17/658) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.026s] ( 18/658) atelier-app health::tests::review_backend_health_reports_missing_role_authors_before_token_lookup
        PASS [   0.027s] ( 19/658) atelier-app health::tests::review_backend_health_reports_provider_success
        PASS [   0.028s] ( 20/658) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.028s] ( 21/658) atelier-app forgejo::tests::lists_top_level_pull_comments
        PASS [   0.011s] ( 22/658) atelier-app pr::tests::parse_pull_request_reference_rejects_mismatched_url_context
        PASS [   0.012s] ( 23/658) atelier-app pr::tests::parse_review_event_rejects_unknown_values
        PASS [   0.026s] ( 24/658) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.015s] ( 25/658) atelier-app pr::tests::parse_pull_request_reference_accepts_number_and_matching_url
        PASS [   0.027s] ( 26/658) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.027s] ( 27/658) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.043s] ( 28/658) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.012s] ( 29/658) atelier-app project_config::tests::forgejo_loader_applies_workflow_role_authors
        PASS [   0.012s] ( 30/658) atelier-app project_config::tests::invalid_forgejo_config_names_and_legacy_role_authors
        PASS [   0.010s] ( 31/658) atelier-app project_config::tests::par
```
