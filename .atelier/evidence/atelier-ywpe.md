---
created_at: "2026-11-09T07:58:24.980808840+00:00"
id: "atelier-ywpe"
evidence_type: "validation"
commit: "3cb961ba9a85378701549135f55c71fdc3238fdd"
result: "pass"
captured_at: "2026-11-09T07:58:24.444452678+00:00"
command: "bash -c 'git remote -v && git ls-remote --heads git@github.com:spadaval/atelier.git master'"
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
title: "origin pushes over SSH here and SSH reaches GitHub (read-only ls-remote)"
updated_at: "2026-11-09T07:58:25.108142270+00:00"
---

## Summary

origin pushes over SSH here and SSH reaches GitHub (read-only ls-remote)

## Command

```console
bash -c 'git remote -v && git ls-remote --heads git@github.com:spadaval/atelier.git master'
```

Exit status: 0

## Stdout

Bytes: 160
SHA-256: 103617e08c479f47642eed3d1a14077ec402ed7b5ad669af99d100380630a09c

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
