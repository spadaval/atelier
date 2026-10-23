---
created_at: "2026-10-23T16:05:21.045222397+00:00"
id: "atelier-wjfa"
evidence_type: "test"
commit: "1ac26495b9012b9615d9eaa522cc72906f4acf96"
result: "pass"
captured_at: "2026-10-23T16:05:18.005852571+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_gate_on_a_record_that_differs_from_head_is_fixed_by_committing_it) | test(land_shows_no_pass_for_an_owner_whose_checkout_record_differs) | test(land_base_mismatch_prints_a_runnable_fix) | test(an_unowned_branch_changing_an_owned_item_points_to_its_branch) | test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run) | test(merge_check_names_every_uncommitted_record_and_path_and_still_runs_the_gates)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-0uaj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0uaj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Findings atelier-6ajc and atelier-9a6e: lines on records that differ from HEAD fix by committing them with no stale pass; land_base_mismatch and owned-item hints print runnable fixes, all run verbatim with the gate rerun"
updated_at: "2026-10-23T16:05:21.045681214+00:00"
---

## Summary

Findings atelier-6ajc and atelier-9a6e: lines on records that differ from HEAD fix by committing them with no stale pass; land_base_mismatch and owned-item hints print runnable fixes, all run verbatim with the gate rerun

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_gate_on_a_record_that_differs_from_head_is_fixed_by_committing_it) | test(land_shows_no_pass_for_an_owner_whose_checkout_record_differs) | test(land_base_mismatch_prints_a_runnable_fix) | test(an_unowned_branch_changing_an_owned_item_points_to_its_branch) | test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run) | test(merge_check_names_every_uncommitted_record_and_path_and_still_runs_the_gates)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1150
SHA-256: 9f1a8b4cb462f48285c6f0977055312c7178c7f7dc8fcc0dbab0fb5b7df8c7de
Test summary: Summary [   0.808s] 6 tests run: 6 passed, 720 skipped
