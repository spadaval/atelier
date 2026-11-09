---
created_at: "2026-11-09T15:16:18.091798693+00:00"
id: "atelier-921m"
evidence_type: "test"
commit: "c5d9cbcf8b4287a64375ff65eae67a7f0a7e8dad"
result: "fail"
captured_at: "2026-11-09T15:16:17.954742802+00:00"
command: "git diff --check master...HEAD"
exit_status: "2"
target:
  kind: "issue"
  id: "atelier-tu8c"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tu8c"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "whitespace check of the branch's diff against master"
updated_at: "2026-11-09T15:16:18.221329298+00:00"
---

## Summary

whitespace check of the branch's diff against master

## Command

```console
git diff --check master...HEAD
```

Exit status: 2

## Stdout

Bytes: 59
Truncated: no
SHA-256: 341e6b4488e0e534a710329c5ef96d66ab3bddcc9ac815d4abf03a4b06851e33

```text
.atelier/issues/atelier-tu8c.md:64: new blank line at EOF.
```

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
