---
created_at: "2026-11-11T08:38:00.053322209+00:00"
id: "atelier-wj1t"
evidence_type: "test"
commit: "f47a181fee4b844ad3afb079ad6d3ff25d83c8c5"
result: "pass"
captured_at: "2026-11-11T08:37:59.482127904+00:00"
command: "desk/service/make-clones.test.sh"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-va99"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-va99"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "make-clones.sh refuses seven paths that resolve inside a scratch projects directory or source, writing nothing"
updated_at: "2026-11-11T08:38:00.231482724+00:00"
---

## Summary

make-clones.sh refuses seven paths that resolve inside a scratch projects directory or source, writing nothing

## Command

```console
desk/service/make-clones.test.sh
```

Exit status: 0

## Stdout

Bytes: 1987
SHA-256: e3da1397829f16f0a7f63b2bf886f72c9a0d1f764a8f7d539012243c77217275

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
