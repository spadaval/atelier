---
created_at: "2026-11-09T17:13:59.182592660+00:00"
id: "atelier-i3sc"
evidence_type: "test"
commit: "eef3a56fbe14ece1694390d496e5b11b2f0ded5b"
result: "pass"
captured_at: "2026-11-09T17:12:48.004497165+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zpmb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zpmb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After the round 2 fixes: full workspace suite passes"
updated_at: "2026-11-09T17:13:59.313978486+00:00"
---

## Summary

After the round 2 fixes: full workspace suite passes

## Command

```console
cargo nextest run -j 4 --build-jobs 4 --workspace
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 141592
SHA-256: 8a5bfc381a9eecdda8ce84847ece0212a049153c1374fac0b68c8f8936b49d99
Test summary: Summary [  66.825s] 1117 tests run: 1117 passed, 0 skipped
