---
created_at: "2026-11-10T19:14:07.976918434+00:00"
id: "atelier-oqv2"
evidence_type: "test"
commit: "88863e85edce1cd566986063fe2bd56692b2e050"
result: "pass"
captured_at: "2026-11-10T19:14:07.816102563+00:00"
command: "bash -c 'git diff --check master... && test -z \"$(git diff --name-only master... -- crates/)\" && echo clean'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-67nn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-67nn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "git diff --check against master, and no file under crates/ changed"
updated_at: "2026-11-10T19:14:08.120692022+00:00"
---

## Summary

git diff --check against master, and no file under crates/ changed

## Command

```console
bash -c 'git diff --check master... && test -z "$(git diff --name-only master... -- crates/)" && echo clean'
```

Exit status: 0

## Stdout

Bytes: 6
SHA-256: 2e22da2ab13713309ac75219e525b8e06ed02f3f1963b8feef203fa25827f93d

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
