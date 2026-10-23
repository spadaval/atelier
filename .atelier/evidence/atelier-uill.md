---
created_at: "2026-10-23T15:45:11.694449207+00:00"
id: "atelier-uill"
evidence_type: "test"
commit: "e52310719a7fa54f70f69632d3b9ad301f7ce855"
result: "pass"
captured_at: "2026-10-23T15:45:09.378828108+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-records -p atelier-cli evidence_summary_text a_rerun_summary sync_rerun sync_reruns"
exit_status: "0"
agent_identity: "claude-implementer-imlh"
target:
  kind: "issue"
  id: "atelier-imlh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-imlh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier-ytks: a legacy nested rerun receipt (atelier-qmti) yields its real summary, a heading inside a Summary is kept, an empty Summary gets no leading space, and merge --sync does not stack the rerun suffix"
updated_at: "2026-10-23T15:45:11.694775575+00:00"
---

## Summary

atelier-ytks: a legacy nested rerun receipt (atelier-qmti) yields its real summary, a heading inside a Summary is kept, an empty Summary gets no leading space, and merge --sync does not stack the rerun suffix

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-records -p atelier-cli evidence_summary_text a_rerun_summary sync_rerun sync_reruns
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1117
SHA-256: 03e98016bf5f31dc755f3cae684611da95fe9cfec3a14131a8f8d87bf6292e87
Test summary: Summary [   0.301s] 5 tests run: 5 passed, 768 skipped
