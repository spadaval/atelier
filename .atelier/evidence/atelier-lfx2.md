---
created_at: "2026-10-23T14:59:40.453803995+00:00"
id: "atelier-lfx2"
evidence_type: "test"
commit: "c0b9a62fbf9ad8139d45113ccd816d55dc6665a7"
result: "pass"
captured_at: "2026-10-23T14:59:38.437055279+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(merge_commit_strategy_proves_the_merged_tree_only_when_every_shell_check_passed) | test(merge_commit_strategy_runs_merge_checks_on_the_merged_tree_and_refuses_a_broken_lockfile)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "## Summary\n\natelier-yjhx: a shell.check failing on the merged tree under merge_commit gives atelier merge <id> --sync, then the check command\n\n## Command\n\n```console\ncargo nextest run -j 4 --workspace --features test-hooks -E 'test(merge_commit_strategy_proves_the_merged_tree_only_when_every_shell_check_passed) | test(merge_commit_strategy_runs_merge_checks_on_the_merged_tree_and_refuses_a_broken_lockfile)'\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nSHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n\n## Stderr\n\nBytes: 732\nSHA-256: e7d6e335079473fec696f5fa547375075181adf76b2c0419b122e85cde9e68c2\nTest summary: Summary [   0.314s] 2 tests run: 2 passed, 896 skipped (rerun after merging master)"
updated_at: "2026-10-23T14:59:40.454075462+00:00"
---

## Summary

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
Test summary: Summary [   0.314s] 2 tests run: 2 passed, 896 skipped (rerun after merging master)

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
SHA-256: 9057b36ccc98f8d2f93d9fda5839caca7a63a9b6b974a2267e5d7ebd19ae48bd
Test summary: Summary [   0.324s] 2 tests run: 2 passed, 910 skipped
