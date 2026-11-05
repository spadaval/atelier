---
created_at: "2026-11-05T00:40:56.605081527+00:00"
id: "atelier-l164"
evidence_type: "test"
commit: "37a116ae12ab443b4e69dfcd63e24c214cc7217f"
result: "pass"
captured_at: "2026-11-05T00:40:51.752215877+00:00"
command: "cargo nextest run -p atelier-cli -E 'test(/a_child_closed_on_/)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-b3qt"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-b3qt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A child closed on its parent's branch is added before its delivery, whether the branch merges into the base or merges the base in first"
updated_at: "2026-11-05T00:40:56.606130233+00:00"
---

## Summary

A child closed on its parent's branch is added before its delivery, whether the branch merges into the base or merges the base in first

## Command

```console
cargo nextest run -p atelier-cli -E 'test(/a_child_closed_on_/)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 719
SHA-256: f4d740cb28ad22df9e4ed2e1f10a9ac9c4fc094617d28cd3cd289fb6066815f3
Test summary: Summary [   1.291s] 2 tests run: 2 passed, 741 skipped
