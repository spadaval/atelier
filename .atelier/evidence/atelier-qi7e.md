---
created_at: "2026-11-09T21:12:34.792055959+00:00"
id: "atelier-qi7e"
evidence_type: "test"
commit: "56472e68d53c1f38d4ca05ad8baaed1aa3257ad3"
result: "pass"
captured_at: "2026-11-09T21:12:25.700261797+00:00"
command: "cargo nextest run -j 4 -p atelier-app -p atelier-site"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "atelier-uma8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uma8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier-app and atelier-site unit tests, including the inbox linking rule and generated TypeScript types"
updated_at: "2026-11-09T21:12:34.922970438+00:00"
---

## Summary

atelier-app and atelier-site unit tests, including the inbox linking rule and generated TypeScript types

## Command

```console
cargo nextest run -j 4 -p atelier-app -p atelier-site
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 17160
SHA-256: 4292cb8cb5420d823ffa141b19d34c482754e9d7a1fea73d99a05c3f9180d53f
Test summary: Summary [   7.448s] 136 tests run: 136 passed, 0 skipped
