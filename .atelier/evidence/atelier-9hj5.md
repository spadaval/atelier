---
created_at: "2026-10-23T15:53:43.127453594+00:00"
id: "atelier-9hj5"
evidence_type: "test"
commit: "fd8ad78e1bfc812396803e7a1f2e6a40784cc95a"
result: "pass"
captured_at: "2026-10-23T15:53:40.475134735+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run) | test(merge_check_names_every_uncommitted_record_and_path_and_still_runs_the_gates) | test(staged_canonical_changes_cannot_hide_behind_restored_worktree_bytes)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-grsi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-grsi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land and merge --check name every uncommitted record with its change (first five, then a count) and the git command that commits them (rerun after merging master)"
updated_at: "2026-10-23T15:53:43.128138503+00:00"
---

## Summary

land and merge --check name every uncommitted record with its change (first five, then a count) and the git command that commits them (rerun after merging master)

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run) | test(merge_check_names_every_uncommitted_record_and_path_and_still_runs_the_gates) | test(staged_canonical_changes_cannot_hide_behind_restored_worktree_bytes)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 825
SHA-256: efc9770821c95229e669e95ba65aee4d7b5882e011f959f88e3e536afe9dce3c
Test summary: Summary [   0.664s] 3 tests run: 3 passed, 719 skipped
