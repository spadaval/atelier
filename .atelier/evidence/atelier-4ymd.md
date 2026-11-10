---
created_at: "2026-11-10T15:25:05.484694838+00:00"
id: "atelier-4ymd"
evidence_type: "test"
commit: "90ef631949114062def96d142f563b18a680812e"
result: "pass"
captured_at: "2026-11-10T15:25:05.328346253+00:00"
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
updated_at: "2026-11-10T15:25:05.626979912+00:00"
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
