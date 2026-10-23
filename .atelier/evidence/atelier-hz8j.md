---
created_at: "2026-10-23T15:47:45.182484764+00:00"
id: "atelier-hz8j"
evidence_type: "test"
commit: "e8805f7963b7c597f8009ce572eeaa1f5537b9ae"
result: "pass"
captured_at: "2026-10-23T15:47:42.486776224+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(merge_names_the_work_branch_and_says_when_the_work_is_already_merged) | test(merge_refusals_and_invalid_policy_carry_one_code_and_one_step)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-yrlj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yrlj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "merge names both branches with git switch, from the base and from another branch, and says an already merged item is merged (exit 0)"
updated_at: "2026-10-23T15:47:45.182792369+00:00"
---

## Summary

merge names both branches with git switch, from the base and from another branch, and says an already merged item is merged (exit 0)

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(merge_names_the_work_branch_and_says_when_the_work_is_already_merged) | test(merge_refusals_and_invalid_policy_carry_one_code_and_one_step)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 711
SHA-256: 56b79993be7d51707d48a6a0c47a1f6046b29dd40de551b3b15b5689e169e6e2
Test summary: Summary [   0.522s] 2 tests run: 2 passed, 714 skipped
