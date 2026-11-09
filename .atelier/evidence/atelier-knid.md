---
created_at: "2026-11-09T07:58:26.102883644+00:00"
id: "atelier-knid"
evidence_type: "test"
commit: "3cb961ba9a85378701549135f55c71fdc3238fdd"
result: "pass"
captured_at: "2026-11-09T07:58:25.689239410+00:00"
command: "bash -c 'git diff --check HEAD~1 HEAD && atelier check atelier-7o4r && atelier check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7o4r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7o4r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Whitespace and record checks pass"
updated_at: "2026-11-09T07:58:26.229674001+00:00"
---

## Summary

Whitespace and record checks pass

## Command

```console
bash -c 'git diff --check HEAD~1 HEAD && atelier check atelier-7o4r && atelier check'
```

Exit status: 0

## Stdout

Bytes: 9228
SHA-256: 0ea115a433e4cb8608b0adde06f437b47f474780a2add7db851cdfc6300c3188

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
