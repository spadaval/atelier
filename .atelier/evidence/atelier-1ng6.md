---
created_at: "2026-10-23T15:47:54.621721448+00:00"
id: "atelier-1ng6"
evidence_type: "test"
commit: "e8805f7963b7c597f8009ce572eeaa1f5537b9ae"
result: "pass"
captured_at: "2026-10-23T15:47:50.295947698+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_fixes_for_open_children_and_a_blocks_link_clear_their_failures) | test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it) | test(merge_check_lists_failures_first_with_fixes_and_collapses_resolved_fix_now_findings)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-r3oy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-r3oy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "fix helper reruns the gate after each fix; blocks unlink and backlog, blocked and review children under a closed owner covered; no reopen fix"
updated_at: "2026-10-23T15:47:54.622036807+00:00"
---

## Summary

fix helper reruns the gate after each fix; blocks unlink and backlog, blocked and review children under a closed owner covered; no reopen fix

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_fixes_for_open_children_and_a_blocks_link_clear_their_failures) | test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it) | test(merge_check_lists_failures_first_with_fixes_and_collapses_resolved_fix_now_findings)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 969
SHA-256: e2799c46d3dd80b5d9fed0120381b68e6f490f18ec207ab2276f8aae45b1ba68
Test summary: Summary [   2.109s] 4 tests run: 4 passed, 712 skipped
