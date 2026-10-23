---
created_at: "2026-10-23T15:02:58.395089543+00:00"
id: "atelier-rduz"
evidence_type: "test"
commit: "0af43aa5f8868a7650477be5687a63b5277367f1"
result: "pass"
captured_at: "2026-10-23T15:02:56.016047898+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it)'"
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
title: "merge --check per-finding and open-item lines hold on the merged tree; an author's quiet disposition prints its ID on stdout and the identity warning on stderr"
updated_at: "2026-10-23T15:02:58.395359055+00:00"
---

## Summary

merge --check per-finding and open-item lines hold on the merged tree; an author's quiet disposition prints its ID on stdout and the identity warning on stderr

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 583
SHA-256: 7801f6ac5cc8a8ddc5db6db1679f571c7b48da8bc0077d42e5f25a8166c78abc
Test summary: Summary [   0.396s] 1 test run: 1 passed, 911 skipped
