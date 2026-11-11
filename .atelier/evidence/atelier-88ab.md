---
created_at: "2026-11-11T07:45:40.971084447+00:00"
id: "atelier-88ab"
evidence_type: "test"
commit: "fe32f541f287f94abbdbe9d52fd13a0d0eb56c72"
result: "fail"
captured_at: "2026-11-11T07:45:40.793311021+00:00"
command: "git diff --check master...HEAD"
exit_status: "2"
target:
  kind: "issue"
  id: "atelier-l2aj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-l2aj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "git diff --check over the branch"
updated_at: "2026-11-11T07:45:41.143964081+00:00"
---

## Summary

git diff --check over the branch

## Command

```console
git diff --check master...HEAD
```

Exit status: 2

## Stdout

Bytes: 59
Truncated: no
SHA-256: cac383f8709c24cb3bbf0f0dc9cf5556e242683ac47c69e4fd21a74a495eb2f7

```text
.atelier/issues/atelier-l2aj.md:90: new blank line at EOF.
```

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
