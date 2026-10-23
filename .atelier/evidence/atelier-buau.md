---
created_at: "2026-10-23T16:21:34.608431049+00:00"
id: "atelier-buau"
evidence_type: "test"
commit: "5223ec700cb57369f273738b41d6dbe26ee1ea4f"
result: "pass"
captured_at: "2026-10-23T16:21:32.103450154+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_changed_review_record_is_restored_and_a_committed_rewrite_is_dropped) | test(review_append_only) | test(append_only)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-0uaj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0uaj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Finding atelier-v4rd: a changed or deleted review record is restored, not committed; a committed rewrite is dropped by reset at the tip or rebase below, each fix run verbatim and the gate rerun"
updated_at: "2026-10-23T16:21:34.608882171+00:00"
---

## Summary

Finding atelier-v4rd: a changed or deleted review record is restored, not committed; a committed rewrite is dropped by reset at the tip or rebase below, each fix run verbatim and the gate rerun

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_changed_review_record_is_restored_and_a_committed_rewrite_is_dropped) | test(review_append_only) | test(append_only)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 571
SHA-256: 7152e395310f4bf535de5bc4e42a70e3aa7dcb4e01aef4864fe750a7d87e0d14
Test summary: Summary [   0.724s] 1 test run: 1 passed, 727 skipped
