---
created_at: "2026-11-04T23:31:27.107148891+00:00"
id: "atelier-isq3"
evidence_type: "test"
commit: "38aca63951526174e3fe732ccca115c6c6a0f7bc"
result: "pass"
captured_at: "2026-11-04T23:31:18.821275218+00:00"
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
updated_at: "2026-11-04T23:31:27.107454381+00:00"
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

Bytes: 3477
SHA-256: 0f836ce805be3f5d4a7b68e2fc6b83400e1d91cdd2d4447c1042a43a7ba97ec4
Test summary: Summary [   1.724s] 23 tests run: 23 passed, 718 skipped
