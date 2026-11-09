---
created_at: "2026-11-09T07:58:22.887262414+00:00"
id: "atelier-l7ky"
evidence_type: "validation"
commit: "3cb961ba9a85378701549135f55c71fdc3238fdd"
result: "pass"
captured_at: "2026-11-09T07:58:22.687298707+00:00"
command: "bash -c '! git grep -n merge-ci-passed -- . '\"'\"':!.atelier'\"'\"' '\"'\"':!docs/adr'\"'\"' '\"'\"':!docs/strategies'\"'\"''"
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
title: "No current file outside .atelier, ADRs and strategies names merge-ci-passed (grep finds nothing)"
updated_at: "2026-11-09T07:58:23.067388382+00:00"
---

## Summary

No current file outside .atelier, ADRs and strategies names merge-ci-passed (grep finds nothing)

## Command

```console
bash -c '! git grep -n merge-ci-passed -- . '"'"':!.atelier'"'"' '"'"':!docs/adr'"'"' '"'"':!docs/strategies'"'"''
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
