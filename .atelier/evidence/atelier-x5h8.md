---
created_at: "2026-11-11T05:36:25.907910195+00:00"
id: "atelier-x5h8"
evidence_type: "test"
commit: "b197657b6241eb4026a47fd3739e0f2f1d4a2e29"
result: "pass"
captured_at: "2026-11-11T05:36:12.080446230+00:00"
command: "bash -c 'cd next && cargo nextest run -j 4 2>&1 | tail -3 && cargo fmt -- --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-l5ai"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-l5ai"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/ suite: 253 of 253 pass, fmt and diff check clean"
updated_at: "2026-11-11T05:36:26.071529939+00:00"
---

## Summary

next/ suite: 253 of 253 pass, fmt and diff check clean

## Command

```console
bash -c 'cd next && cargo nextest run -j 4 2>&1 | tail -3 && cargo fmt -- --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 214
SHA-256: c63a0ce41cc5ca09ea718a5f590291c68d5230d112b325621a484f790ce02912
Test summary: Summary [  13.186s] 253 tests run: 253 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
