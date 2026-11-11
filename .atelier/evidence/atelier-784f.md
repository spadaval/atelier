---
created_at: "2026-11-11T01:10:17.357986209+00:00"
id: "atelier-784f"
evidence_type: "test"
commit: "38cae6b79be7c5194494f05dab8256fd06cb013c"
result: "pass"
captured_at: "2026-11-11T01:10:17.191276433+00:00"
command: "bash -c 'test -z \"$(git diff --stat master...HEAD -- crates/)\" && git diff --check master...HEAD'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-efsl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-efsl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "No file under crates/ changed, and no whitespace errors"
updated_at: "2026-11-11T01:10:17.509847484+00:00"
---

## Summary

No file under crates/ changed, and no whitespace errors

## Command

```console
bash -c 'test -z "$(git diff --stat master...HEAD -- crates/)" && git diff --check master...HEAD'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
