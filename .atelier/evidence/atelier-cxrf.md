---
created_at: "2026-10-23T15:54:01.917894851+00:00"
id: "atelier-cxrf"
evidence_type: "test"
commit: "fd8ad78e1bfc812396803e7a1f2e6a40784cc95a"
result: "pass"
captured_at: "2026-10-23T15:53:59.368306435+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(merge_names_the_work_branch_and_says_when_the_work_is_already_merged) | test(merge_refusals_and_invalid_policy_carry_one_code_and_one_step)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "merge names both branches with git switch, from the base and from another branch, and says an already merged item is merged (exit 0) (rerun after merging master)"
updated_at: "2026-10-23T15:54:01.918533974+00:00"
---

## Summary

merge names both branches with git switch, from the base and from another branch, and says an already merged item is merged (exit 0) (rerun after merging master)

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
SHA-256: 46ff89f12a2d1d36c470fc6a2bc52d5c8f3bae21fb0d5dd58266d220a343b3d4
Test summary: Summary [   0.549s] 2 tests run: 2 passed, 720 skipped
