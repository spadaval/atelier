---
created_at: "2026-11-05T01:46:31.993703071+00:00"
id: "atelier-am1a"
evidence_type: "test"
commit: "529a6ce232ba30c4bc1585aaf7c984982b133bf3"
result: "pass"
captured_at: "2026-11-05T01:46:25.057929842+00:00"
command: "cargo nextest run -p atelier-cli brief::"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mg7w"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mg7w"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "With an approval recorded, both forms of the brief offer moving the plan on instead of approving it"
updated_at: "2026-11-05T01:46:31.994178538+00:00"
---

## Summary

With an approval recorded, both forms of the brief offer moving the plan on instead of approving it

## Command

```console
cargo nextest run -p atelier-cli brief::
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 3948
SHA-256: 1f0c2731f646476d179282d09752657d7a5877cc81f1d0a4fb7f2d37d4252f5a
Test summary: Summary [   2.856s] 27 tests run: 27 passed, 724 skipped
