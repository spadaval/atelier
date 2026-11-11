---
created_at: "2026-11-11T00:12:13.944933179+00:00"
id: "atelier-66ds"
evidence_type: "test"
commit: "25f7c4febb03d8d2a5a029d604d224fcc1e9eb32"
result: "pass"
captured_at: "2026-11-11T00:12:13.407210486+00:00"
command: "sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-nafc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nafc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "fmt and clippy -D warnings on next/"
updated_at: "2026-11-11T00:12:14.097878113+00:00"
---

## Summary

fmt and clippy -D warnings on next/

## Command

```console
sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 72
SHA-256: c92508a29848820f033cd2ea015c85dde0953c88b07cc967ce34b7c08f610549
