---
created_at: "2026-11-09T17:18:21.998152145+00:00"
id: "atelier-4jw4"
evidence_type: "test"
commit: "eef3a56fbe14ece1694390d496e5b11b2f0ded5b"
result: "pass"
captured_at: "2026-11-09T17:18:21.844218880+00:00"
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
title: "After the round 2 fixes: the whole branch passes git diff --check outside the tracker records"
updated_at: "2026-11-09T17:18:22.131716325+00:00"
---

## Summary

After the round 2 fixes: the whole branch passes git diff --check outside the tracker records

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
