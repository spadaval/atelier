---
created_at: "2026-11-09T17:03:40.994415107+00:00"
id: "atelier-mlu1"
evidence_type: "test"
commit: "594dc0f8fc5fd3a5955733f4aa4834cb3a47ce98"
result: "pass"
captured_at: "2026-11-09T17:03:40.850309090+00:00"
command: "git diff --check 94990dd3 HEAD -- . ':!.atelier'"
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
title: "The whole branch passes git diff --check outside the tracker records, generated TypeScript included"
updated_at: "2026-11-09T17:03:41.132196763+00:00"
---

## Summary

The whole branch passes git diff --check outside the tracker records, generated TypeScript included

## Command

```console
git diff --check 94990dd3 HEAD -- . ':!.atelier'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
