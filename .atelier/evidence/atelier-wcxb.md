---
created_at: "2026-10-23T17:58:44.702785620+00:00"
id: "atelier-wcxb"
evidence_type: "test"
commit: "cd790a67a0fd34914afe25337408d4b88a439af4"
result: "pass"
captured_at: "2026-10-23T17:57:59.026978610+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "independent validator (Claude)"
target:
  kind: "issue"
  id: "atelier-1o56"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1o56"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks on master 5abb3651 (validation branch cd790a67): merge_gates 87, session 136, records_evidence 52, review_views 6 tests among 953; none skipped"
updated_at: "2026-10-23T17:58:44.703429922+00:00"
---

## Summary

Full workspace suite with test-hooks on master 5abb3651 (validation branch cd790a67): merge_gates 87, session 136, records_evidence 52, review_views 6 tests among 953; none skipped

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 119668
SHA-256: fa2d36729b91ae3fef74d07a86c6e0b37a3d446cacb27cae625434eb2fa3f7bf
Test summary: Summary [  43.433s] 953 tests run: 953 passed (1 slow), 0 skipped
