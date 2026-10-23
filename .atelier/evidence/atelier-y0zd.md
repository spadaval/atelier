---
created_at: "2026-10-23T14:57:19.074349238+00:00"
id: "atelier-y0zd"
evidence_type: "test"
commit: "06272b9d4b6401c8d7fe6c7561257da0b2c9446d"
result: "pass"
captured_at: "2026-10-23T14:57:17.050300031+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(merge_commit_strategy_proves_the_merged_tree_only_when_every_shell_check_passed) | test(merge_commit_strategy_runs_merge_checks_on_the_merged_tree_and_refuses_a_broken_lockfile)'"
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
title: "atelier-yjhx: a shell.check failing on the merged tree under merge_commit gives atelier merge <id> --sync, then the check command"
updated_at: "2026-10-23T14:57:19.074624361+00:00"
---

## Summary

atelier-yjhx: a shell.check failing on the merged tree under merge_commit gives atelier merge <id> --sync, then the check command

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(merge_commit_strategy_proves_the_merged_tree_only_when_every_shell_check_passed) | test(merge_commit_strategy_runs_merge_checks_on_the_merged_tree_and_refuses_a_broken_lockfile)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 732
SHA-256: e7d6e335079473fec696f5fa547375075181adf76b2c0419b122e85cde9e68c2
Test summary: Summary [   0.314s] 2 tests run: 2 passed, 896 skipped
