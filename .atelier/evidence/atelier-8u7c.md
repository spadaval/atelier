---
created_at: "2026-11-10T17:45:21.833889832+00:00"
id: "atelier-8u7c"
evidence_type: "test"
commit: "edc10b23701bdc678dfed0f407ecb0e5e258c4e6"
result: "pass"
captured_at: "2026-11-10T17:45:21.656042010+00:00"
command: "bash -c 'git diff --check master...HEAD && test -z \"$(git diff --stat master...HEAD -- crates/)\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-syyj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-syyj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "git diff --check clean; no change under crates/"
updated_at: "2026-11-10T17:45:21.995244316+00:00"
---

## Summary

git diff --check clean; no change under crates/

## Command

```console
bash -c 'git diff --check master...HEAD && test -z "$(git diff --stat master...HEAD -- crates/)"'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
