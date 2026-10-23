---
created_at: "2026-10-23T15:47:36.993213822+00:00"
id: "atelier-tkyi"
evidence_type: "test"
commit: "e8805f7963b7c597f8009ce572eeaa1f5537b9ae"
result: "pass"
captured_at: "2026-10-23T15:47:34.253448074+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run) | test(merge_check_names_every_uncommitted_record_and_path_and_still_runs_the_gates) | test(staged_canonical_changes_cannot_hide_behind_restored_worktree_bytes)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
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
title: "land and merge --check name every uncommitted record with its change (first five, then a count) and the git command that commits them"
updated_at: "2026-10-23T15:47:36.993516006+00:00"
---

## Summary

land and merge --check name every uncommitted record with its change (first five, then a count) and the git command that commits them

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
SHA-256: c54fdd83145adea09af8ad50d7795485576cf60848f29fca09367a2da431159d
Test summary: Summary [   0.663s] 3 tests run: 3 passed, 713 skipped
