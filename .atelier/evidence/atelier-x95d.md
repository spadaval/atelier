---
created_at: "2026-10-23T15:47:39.713283586+00:00"
id: "atelier-x95d"
evidence_type: "test"
commit: "e8805f7963b7c597f8009ce572eeaa1f5537b9ae"
result: "pass"
captured_at: "2026-10-23T15:47:37.093865444+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run) | test(hidden_dirty_code_is_an_honest_active_handoff_but_not_a_completion)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-7ts2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7ts2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land_completion_dirty names the dirty paths (first five, then a count) with commit or stash fixes that run verbatim"
updated_at: "2026-10-23T15:47:39.713566374+00:00"
---

## Summary

land_completion_dirty names the dirty paths (first five, then a count) with commit or stash fixes that run verbatim

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run) | test(hidden_dirty_code_is_an_honest_active_handoff_but_not_a_completion)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 691
SHA-256: 337690e7b47d2687a08f160fc8f38319ba92ca6d586fb2a43ab07a85664f0969
Test summary: Summary [   0.445s] 2 tests run: 2 passed, 714 skipped
