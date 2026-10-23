---
created_at: "2026-10-23T17:36:23.322374507+00:00"
id: "atelier-v7id"
evidence_type: "test"
commit: "2957f128bf09b4ff608a939d3a8d3fe80f407ced"
result: "pass"
captured_at: "2026-10-23T17:36:21.033729476+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_policy_from_a_newer_atelier_says_to_reinstall) | test(a_resolved_fix_now_awaiting_approval_blocks_no_turn)'"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-buqu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-buqu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Newer workflow policy says only to reinstall in land and fallback commands; resolved fix_now awaiting approval is a pending review"
updated_at: "2026-10-23T17:36:23.322626667+00:00"
---

## Summary

Newer workflow policy says only to reinstall in land and fallback commands; resolved fix_now awaiting approval is a pending review

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_policy_from_a_newer_atelier_says_to_reinstall) | test(a_resolved_fix_now_awaiting_approval_blocks_no_turn)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 654
SHA-256: 553ae9f7472c267ad9608685779531d99e37c1a6b9d3c0949a5bf3cc13539346
Test summary: Summary [   0.478s] 2 tests run: 2 passed, 743 skipped
