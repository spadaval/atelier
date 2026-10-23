---
created_at: "2026-10-23T15:53:40.127623300+00:00"
id: "atelier-i3c7"
evidence_type: "test"
commit: "fd8ad78e1bfc812396803e7a1f2e6a40784cc95a"
result: "pass"
captured_at: "2026-10-23T15:53:37.643107790+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run) | test(hidden_dirty_code_is_an_honest_active_handoff_but_not_a_completion)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "land_completion_dirty names the dirty paths (first five, then a count) with commit or stash fixes that run verbatim (rerun after merging master)"
updated_at: "2026-10-23T15:53:40.128298010+00:00"
---

## Summary

land_completion_dirty names the dirty paths (first five, then a count) with commit or stash fixes that run verbatim (rerun after merging master)

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
SHA-256: 2a6d56fd2b69f22afd41b03f6f8f7220c5b255c6ef3e965dc561effced94b406
Test summary: Summary [   0.451s] 2 tests run: 2 passed, 720 skipped
