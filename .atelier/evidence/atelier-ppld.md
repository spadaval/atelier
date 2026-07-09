---
created_at: "2026-07-09T16:40:48.626469269+00:00"
id: "atelier-ppld"
evidence_type: "validation"
captured_at: "2026-07-09T16:40:45.316969818+00:00"
command: "bash -lc 'set -euo pipefail; cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work; echo \"PASS: isolated temp repositories cover flat parent/child/mission/epic/standalone/blocked/done/empty inventory states; metadata filters, quiet, positive limit, unknown category and removed flag errors; mission/epic/child/direct/unassigned/blocked/done/empty overview states; collapsed leaves; progress, blockers and drilldowns; done omission and --all; captured and NO_COLOR semantics.\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-g5fl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-g5fl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail; cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work; echo \"PASS: isolated temp repositories cover flat parent/child/mission/epic/standalone/blocked/done/empty inventory states; metadata filters, quiet, positive limit, unknown category and removed flag errors; mission/epic/child/direct/unassigned/blocked/done/empty overview states; collapsed leaves; progress, blockers and drilldowns; done omission and --all; captured and NO_COLOR semantics.\"'"
updated_at: "2026-07-09T16:40:48.628834135+00:00"
---

## Summary

bash -lc 'set -euo pipefail; cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work; echo "PASS: isolated temp repositories cover flat parent/child/mission/epic/standalone/blocked/done/empty inventory states; metadata filters, quiet, positive limit, unknown category and removed flag errors; mission/epic/child/direct/unassigned/blocked/done/empty overview states; collapsed leaves; progress, blockers and drilldowns; done omission and --all; captured and NO_COLOR semantics."'

## Command

```console
bash -lc 'set -euo pipefail; cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work; echo "PASS: isolated temp repositories cover flat parent/child/mission/epic/standalone/blocked/done/empty inventory states; metadata filters, quiet, positive limit, unknown category and removed flag errors; mission/epic/child/direct/unassigned/blocked/done/empty overview states; collapsed leaves; progress, blockers and drilldowns; done omission and --all; captured and NO_COLOR semantics."'
```
Exit status: 0

## Stdout

Bytes: 385
Truncated: no

```text
PASS: isolated temp repositories cover flat parent/child/mission/epic/standalone/blocked/done/empty inventory states; metadata filters, quiet, positive limit, unknown category and removed flag errors; mission/epic/child/direct/unassigned/blocked/done/empty overview states; collapsed leaves; progress, blockers and drilldowns; done omission and --all; captured and NO_COLOR semantics.
```

## Stderr

Bytes: 951
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-coordination/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.23s
────────────
 Nextest run ID efe54a76-5f9a-4b20-9d79-952e14dd012c with nextest profile: default
    Starting 3 tests across 4 binaries (470 tests skipped)
        PASS [   0.340s] (1/3) atelier-cli::cli_integration mission_cache_worktree::test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work
        PASS [   0.447s] (2/3) atelier-cli::cli_integration issues::test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags
        PASS [   0.543s] (3/3) atelier-cli::cli_integration mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output
────────────
     Summary [   0.544s] 3 tests run: 3 passed, 470 skipped
```
