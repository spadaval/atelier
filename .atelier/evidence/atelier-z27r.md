---
created_at: "2026-11-10T15:36:11.805461467+00:00"
id: "atelier-z27r"
evidence_type: "test"
commit: "1a14e2ff239af8db7aa0bd143e0ddb479084d583"
result: "pass"
captured_at: "2026-11-10T15:36:11.649576517+00:00"
command: "bash -c 'base=$(git merge-base master HEAD); test -z \"$(git diff --name-only $base HEAD -- crates/)\" && git diff --check $base HEAD'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mal9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mal9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "no file under crates/ changed, and git diff --check is clean"
updated_at: "2026-11-10T15:36:11.946442972+00:00"
---

## Summary

no file under crates/ changed, and git diff --check is clean

## Command

```console
bash -c 'base=$(git merge-base master HEAD); test -z "$(git diff --name-only $base HEAD -- crates/)" && git diff --check $base HEAD'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
