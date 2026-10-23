---
created_at: "2026-10-23T17:31:29.018174401+00:00"
id: "atelier-qr4q"
evidence_type: "test"
commit: "d5a4bebcd353188a3cc4880716c0732a5f494519"
result: "pass"
captured_at: "2026-10-23T17:31:26.842561022+00:00"
command: "cargo nextest run -j 4 -p atelier-cli -p atelier-records -p atelier-workflow --features test-hooks -E 'test(newer) | test(version_two_rejects_old_versions) | test(test_queries_fail_naming_the_invalid_record_file) | test(test_lint_has_stable_diagnostics_for_hard_invalid_markdown_records)'"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-u4ql"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u4ql"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Higher schema_version in issue, evidence, review and workflow policy says to reinstall in status, its hook context, land, the Stop hook and other commands; lower versions keep the old error"
updated_at: "2026-10-23T17:31:29.018640401+00:00"
---

## Summary

Higher schema_version in issue, evidence, review and workflow policy says to reinstall in status, its hook context, land, the Stop hook and other commands; lower versions keep the old error

## Command

```console
cargo nextest run -j 4 -p atelier-cli -p atelier-records -p atelier-workflow --features test-hooks -E 'test(newer) | test(version_two_rejects_old_versions) | test(test_queries_fail_naming_the_invalid_record_file) | test(test_lint_has_stable_diagnostics_for_hard_invalid_markdown_records)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1511
SHA-256: 395cffada018694a43c7c615f30e04370e4c8bfeaa38901d59819b0c7a688b76
Test summary: Summary [   0.409s] 9 tests run: 9 passed, 825 skipped
