---
created_at: "2026-10-23T17:31:37.247806056+00:00"
id: "atelier-c0nq"
evidence_type: "test"
commit: "d5a4bebcd353188a3cc4880716c0732a5f494519"
result: "pass"
captured_at: "2026-10-23T17:31:34.256421627+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(an_operation_left_in_progress_is_reported_before_any_gate) | test(leaving_out_a_merge_that_rewrote_a_review_warns_that_its_work_goes_too)'"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-zegg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zegg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Land names a conflicted cherry-pick or revert with its abort, which the helper runs; leaving out a merge that rewrote a review warns its merged work is dropped, checked by running the fix"
updated_at: "2026-10-23T17:31:37.248068135+00:00"
---

## Summary

Land names a conflicted cherry-pick or revert with its abort, which the helper runs; leaving out a merge that rewrote a review warns its merged work is dropped, checked by running the fix

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(an_operation_left_in_progress_is_reported_before_any_gate) | test(leaving_out_a_merge_that_rewrote_a_review_warns_that_its_work_goes_too)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 683
SHA-256: 79a1874313381d771f884342054e445f24e50f729521a11e2c04b4ab525326c9
Test summary: Summary [   1.189s] 2 tests run: 2 passed, 742 skipped
