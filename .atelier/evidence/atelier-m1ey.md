---
created_at: "2026-10-23T17:31:33.981531886+00:00"
id: "atelier-m1ey"
evidence_type: "test"
commit: "d5a4bebcd353188a3cc4880716c0732a5f494519"
result: "pass"
captured_at: "2026-10-23T17:31:31.763738040+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_pending_review_on_the_owners_branch_blocks_no_turn)'"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-i80r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-i80r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A pending review alone on the owner's branch gives a non-blocking systemMessage while land refuses; plus another failure or with an open finding it blocks"
updated_at: "2026-10-23T17:31:33.981977307+00:00"
---

## Summary

A pending review alone on the owner's branch gives a non-blocking systemMessage while land refuses; plus another failure or with an open finding it blocks

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_pending_review_on_the_owners_branch_blocks_no_turn)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 553
SHA-256: 7c77c29f9ea09c76e87710697d584f606bb1bab3b257ce32dd4c5ed573775af2
Test summary: Summary [   0.434s] 1 test run: 1 passed, 743 skipped
