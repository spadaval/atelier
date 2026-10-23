---
created_at: "2026-10-23T14:57:16.955516170+00:00"
id: "atelier-49q4"
evidence_type: "test"
commit: "06272b9d4b6401c8d7fe6c7561257da0b2c9446d"
result: "pass"
captured_at: "2026-10-23T14:57:14.734291188+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it) | test(merge_commit_strategy_refuses_a_merge_that_breaks_a_claim_the_base_completed)'"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-h1nc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h1nc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier-5nk6: open scoped items get the workflow's closing transitions in order; an open todo blocker gets atelier start plus the unlink naming the declaring record"
updated_at: "2026-10-23T14:57:16.955993421+00:00"
---

## Summary

atelier-5nk6: open scoped items get the workflow's closing transitions in order; an open todo blocker gets atelier start plus the unlink naming the declaring record

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it) | test(merge_commit_strategy_refuses_a_merge_that_breaks_a_claim_the_base_completed)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 843
SHA-256: 3b47e534a9341d655ad91a226c0fdd28314e49af09a6660a4ec9064ad1e8ea08
Test summary: Summary [   0.520s] 3 tests run: 3 passed, 895 skipped
