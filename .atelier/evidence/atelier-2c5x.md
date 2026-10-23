---
created_at: "2026-10-23T14:27:44.724116547+00:00"
id: "atelier-2c5x"
evidence_type: "test"
commit: "22f79132550a8519f19b8e5bf49f5e783535163e"
result: "pass"
captured_at: "2026-10-23T14:27:04.739771898+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-dg3h"
target:
  kind: "issue"
  id: "atelier-dg3h"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-dg3h"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite passes with test-hooks"
updated_at: "2026-10-23T14:27:44.724749918+00:00"
---

## Summary

Full workspace suite passes with test-hooks

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 113079
SHA-256: 17b4ba0a1bc11858aadd07c975aafdebba0409a3d4128a83f073c393785957cf
Test summary: Summary [  37.812s] 900 tests run: 900 passed (1 slow), 0 skipped
