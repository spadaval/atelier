---
created_at: "2026-11-10T15:38:55.482990899+00:00"
id: "atelier-h7ob"
evidence_type: "test"
commit: "cd0d159306365ea3d185f65c9752480b061bcd90"
result: "pass"
captured_at: "2026-11-10T15:38:55.321861304+00:00"
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
updated_at: "2026-11-10T15:38:55.622145649+00:00"
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
