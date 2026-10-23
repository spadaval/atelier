---
created_at: "2026-10-23T17:31:31.486584756+00:00"
id: "atelier-sycs"
evidence_type: "test"
commit: "d5a4bebcd353188a3cc4880716c0732a5f494519"
result: "pass"
captured_at: "2026-10-23T17:31:29.294049170+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(uncommitted_records_alone_block_no_turn_on_the_base) | test(documented_hook_events_use_nested_cwd_and_stop_loop_protection)'"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-mp6v"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mp6v"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "On the base, uncommitted records alone give a non-blocking systemMessage while land refuses; with another failure on the base, or on a work branch, the hook blocks; printed fix runs"
updated_at: "2026-10-23T17:31:31.487025849+00:00"
---

## Summary

On the base, uncommitted records alone give a non-blocking systemMessage while land refuses; with another failure on the base, or on a work branch, the hook blocks; printed fix runs

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(uncommitted_records_alone_block_no_turn_on_the_base) | test(documented_hook_events_use_nested_cwd_and_stop_loop_protection)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 669
SHA-256: 2bf6d254fd86c4c341476371e8376c294336946ccc328535398c9ae38dd4234b
Test summary: Summary [   0.420s] 2 tests run: 2 passed, 742 skipped
