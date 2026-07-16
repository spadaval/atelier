---
created_at: "2026-07-09T16:52:49.333206853+00:00"
id: "atelier-4k8q"
evidence_type: "validation"
captured_at: "2026-07-09T16:52:49.148599251+00:00"
command: "bash -lc 'set -euo pipefail; D=$(mktemp -d); trap \"rm -rf $D\" EXIT; test \"$(git rev-parse HEAD)\" = \"a9763d405f754d000c425f68841e19da26c157aa\"; cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views >\"$D/focused\" 2>&1; grep -q \"4 tests run: 4 passed\" \"$D/focused\"; BIN=$PWD/target/debug/atelier; \"$BIN\" issue list --help | grep -q \"List issue records as generic inventory\"; \"$BIN\" work missions --help | grep -q \"Show the Mission Overview across current missions\"; \"$BIN\" man manager >\"$D/manager\"; grep -q \"atelier work missions\" \"$D/manager\"; grep -q \"atelier man work-model\" \"$D/manager\"; rg -q \"Mission Overview\" docs/product/issue-inventory-and-mission-overview.md docs/product/cli-surface.md docs/product/human-cli-output.md; cargo nextest run >\"$D/full\" 2>&1; grep -q \"704 tests run: 704 passed, 0 skipped\" \"$D/full\"; cargo fmt -- --check; git diff --check; \"$BIN\" check atelier-c0mp >\"$D/check-mission\"; \"$BIN\" check >\"$D/check-repo\"; grep -q \"Lint passed\" \"$D/check-mission\"; grep -q \"Lint passed\" \"$D/check-repo\"; echo \"PASS: atelier-g5fl remains valid after master integration at a9763d40. Mission-critical scenarios: 4/4; full suite: 704 passed, 0 skipped; inventory/overview help, docs, manager Mission Overview plus man work-model guidance, formatting, diff, mission lint, and repository lint all pass.\"; grep \"Summary\" \"$D/focused\" | tail -1; grep \"Summary\" \"$D/full\" | tail -1'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-c0mp"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c0mp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail; D=$(mktemp -d); trap \"rm -rf $D\" EXIT; test \"$(git rev-parse HEAD)\" = \"a9763d405f754d000c425f68841e19da26c157aa\"; cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views >\"$D/focused\" 2>&1; grep -q \"4 tests run: 4 passed\" \"$D/focused\"; BIN=$PWD/target/debug/atelier; \"$BIN\" issue list --help | grep -q \"List issue records as generic inventory\"; \"$BIN\" work missions --help | grep -q \"Show the Mission Overview across current missions\"; \"$BIN\" man manager >\"$D/manager\"; grep -q \"atelier work missions\" \"$D/manager\"; grep -q \"atelier man work-model\" \"$D/manager\"; rg -q \"Mission Overview\" docs/product/issue-inventory-and-mission-overview.md docs/product/cli-surface.md docs/product/human-cli-output.md; cargo nextest run >\"$D/full\" 2>&1; grep -q \"704 tests run: 704 passed, 0 skipped\" \"$D/full\"; cargo fmt -- --check; git diff --check; \"$BIN\" check atelier-c0mp >\"$D/check-mission\"; \"$BIN\" check >\"$D/check-repo\"; grep -q \"Lint passed\" \"$D/check-mission\"; grep -q \"Lint passed\" \"$D/check-repo\"; echo \"PASS: atelier-g5fl remains valid after master integration at a9763d40. Mission-critical scenarios: 4/4; full suite: 704 passed, 0 skipped; inventory/overview help, docs, manager Mission Overview plus man work-model guidance, formatting, diff, mission lint, and repository lint all pass.\"; grep \"Summary\" \"$D/focused\" | tail -1; grep \"Summary\" \"$D/full\" | tail -1'"
updated_at: "2026-07-09T16:52:49.339655558+00:00"
---

## Summary

bash -lc 'set -euo pipefail; D=$(mktemp -d); trap "rm -rf $D" EXIT; test "$(git rev-parse HEAD)" = "a9763d405f754d000c425f68841e19da26c157aa"; cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views >"$D/focused" 2>&1; grep -q "4 tests run: 4 passed" "$D/focused"; BIN=$PWD/target/debug/atelier; "$BIN" issue list --help | grep -q "List issue records as generic inventory"; "$BIN" work missions --help | grep -q "Show the Mission Overview across current missions"; "$BIN" man manager >"$D/manager"; grep -q "atelier work missions" "$D/manager"; grep -q "atelier man work-model" "$D/manager"; rg -q "Mission Overview" docs/product/issue-inventory-and-mission-overview.md docs/product/cli-surface.md docs/product/human-cli-output.md; cargo nextest run >"$D/full" 2>&1; grep -q "704 tests run: 704 passed, 0 skipped" "$D/full"; cargo fmt -- --check; git diff --check; "$BIN" check atelier-c0mp >"$D/check-mission"; "$BIN" check >"$D/check-repo"; grep -q "Lint passed" "$D/check-mission"; grep -q "Lint passed" "$D/check-repo"; echo "PASS: atelier-g5fl remains valid after master integration at a9763d40. Mission-critical scenarios: 4/4; full suite: 704 passed, 0 skipped; inventory/overview help, docs, manager Mission Overview plus man work-model guidance, formatting, diff, mission lint, and repository lint all pass."; grep "Summary" "$D/focused" | tail -1; grep "Summary" "$D/full" | tail -1'

## Command

```console
bash -lc 'set -euo pipefail; D=$(mktemp -d); trap "rm -rf $D" EXIT; test "$(git rev-parse HEAD)" = "a9763d405f754d000c425f68841e19da26c157aa"; cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views >"$D/focused" 2>&1; grep -q "4 tests run: 4 passed" "$D/focused"; BIN=$PWD/target/debug/atelier; "$BIN" issue list --help | grep -q "List issue records as generic inventory"; "$BIN" work missions --help | grep -q "Show the Mission Overview across current missions"; "$BIN" man manager >"$D/manager"; grep -q "atelier work missions" "$D/manager"; grep -q "atelier man work-model" "$D/manager"; rg -q "Mission Overview" docs/product/issue-inventory-and-mission-overview.md docs/product/cli-surface.md docs/product/human-cli-output.md; cargo nextest run >"$D/full" 2>&1; grep -q "704 tests run: 704 passed, 0 skipped" "$D/full"; cargo fmt -- --check; git diff --check; "$BIN" check atelier-c0mp >"$D/check-mission"; "$BIN" check >"$D/check-repo"; grep -q "Lint passed" "$D/check-mission"; grep -q "Lint passed" "$D/check-repo"; echo "PASS: atelier-g5fl remains valid after master integration at a9763d40. Mission-critical scenarios: 4/4; full suite: 704 passed, 0 skipped; inventory/overview help, docs, manager Mission Overview plus man work-model guidance, formatting, diff, mission lint, and repository lint all pass."; grep "Summary" "$D/focused" | tail -1; grep "Summary" "$D/full" | tail -1'
```
Exit status: 1

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 0
Truncated: no

```text
```
