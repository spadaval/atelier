---
created_at: "2026-10-23T16:06:07.671036193+00:00"
id: "atelier-bcau"
evidence_type: "test"
commit: "1ac26495b9012b9615d9eaa522cc72906f4acf96"
result: "pass"
captured_at: "2026-10-23T16:05:21.158572680+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
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
title: "Full workspace suite with test hooks passes after the atelier-6ajc and atelier-9a6e fixes"
updated_at: "2026-10-23T16:06:07.671301678+00:00"
---

## Summary

Full workspace suite with test hooks passes after the atelier-6ajc and atelier-9a6e fixes

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 117179
SHA-256: a123f36cbb38d3cd1c8cd8b9baa56a2e5f083da5c466438830a29dce1e79efec
Test summary: Summary [  44.287s] 932 tests run: 932 passed (1 slow), 0 skipped
