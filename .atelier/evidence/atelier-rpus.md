---
created_at: "2026-11-11T07:45:46.714330604+00:00"
id: "atelier-rpus"
evidence_type: "test"
commit: "86192183a5b1a26eb5e9040db199a7a24c9b63c2"
result: "pass"
captured_at: "2026-11-11T07:45:46.539207033+00:00"
command: "git diff --check master...HEAD"
exit_status: "0"
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
title: "git diff --check over the branch, after the trim"
updated_at: "2026-11-11T07:45:46.881433329+00:00"
---

## Summary

git diff --check over the branch, after the trim

## Command

```console
git diff --check master...HEAD
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
