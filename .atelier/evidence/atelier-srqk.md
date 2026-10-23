---
created_at: "2026-10-23T17:31:26.567116626+00:00"
id: "atelier-srqk"
evidence_type: "test"
commit: "d5a4bebcd353188a3cc4880716c0732a5f494519"
result: "pass"
captured_at: "2026-10-23T17:31:24.076566941+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(a_git_failure_during_land_is_not_called_uncommitted_records)'"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-467y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-467y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Hook block reason equals land's refusal with one recovery step per failure; a Git failure during land is not labelled land_records_uncommitted"
updated_at: "2026-10-23T17:31:26.567380138+00:00"
---

## Summary

Hook block reason equals land's refusal with one recovery step per failure; a Git failure during land is not labelled land_records_uncommitted

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(a_git_failure_during_land_is_not_called_uncommitted_records)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 687
SHA-256: 88e54141a2e9bb8d23faf3b80e625d69d12f1383591407ac8718b9b55bc72c0e
Test summary: Summary [   0.700s] 2 tests run: 2 passed, 742 skipped
