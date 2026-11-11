---
created_at: "2026-11-11T00:11:30.229870076+00:00"
id: "atelier-6syc"
evidence_type: "validation"
commit: "363378a7a27f65b3aec3c1be416557e71f5f7944"
result: "pass"
captured_at: "2026-11-11T00:11:30.023135978+00:00"
command: "bash -c 'git diff --check master...HEAD && test -z \"$(git diff --stat master...HEAD -- crates/)\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zrss"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zrss"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "git diff --check is clean and git diff --stat master...HEAD -- crates/ is empty"
updated_at: "2026-11-11T00:11:30.382125406+00:00"
---

## Summary

git diff --check is clean and git diff --stat master...HEAD -- crates/ is empty

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
