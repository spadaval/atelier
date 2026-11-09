---
created_at: "2026-11-09T16:49:36.067788168+00:00"
id: "atelier-grj3"
evidence_type: "test"
commit: "b7c839c1b46c70f668c05c810bb0bdc3127944d8"
result: "pass"
captured_at: "2026-11-09T16:48:11.645893059+00:00"
command: "nice -n 5 scripts/smoke-repos.sh"
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
title: "smoke-repos: this tree's binary against every local Atelier repository (check, doctor, status, work missions, review request) shows no regression from the installed binary"
updated_at: "2026-11-09T16:49:36.198306462+00:00"
---

## Summary

smoke-repos: this tree's binary against every local Atelier repository (check, doctor, status, work missions, review request) shows no regression from the installed binary

## Command

```console
nice -n 5 scripts/smoke-repos.sh
```

Exit status: 0

## Stdout

Bytes: 2517
SHA-256: b25f41f1c29cf7ff3dd7d413f0e3e563894a8ecbb0ed0c6a550490710ba08adf

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
