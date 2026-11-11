---
created_at: "2026-11-11T08:05:00.600161449+00:00"
id: "atelier-avcv"
evidence_type: "test"
commit: "c58e0b76314be2ecd4fae80c864e2733cda80b52"
result: "pass"
captured_at: "2026-11-11T08:04:57.174723444+00:00"
command: "sh -c 'python3 -m unittest scripts/cutover/test_convert_records.py && scripts/ci-jobs.test.sh'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-u1ko"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u1ko"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The converter's tests with the binary at its new path, and scripts/ci-jobs.sh's test"
updated_at: "2026-11-11T08:05:00.793301132+00:00"
---

## Summary

The converter's tests with the binary at its new path, and scripts/ci-jobs.sh's test

## Command

```console
sh -c 'python3 -m unittest scripts/cutover/test_convert_records.py && scripts/ci-jobs.test.sh'
```

Exit status: 0

## Stdout

Bytes: 460
SHA-256: cddee23505990cf6e679d74a6f27eb966bfe716c826a64738d35e2d35c8d7a9b

## Stderr

Bytes: 115
SHA-256: 563f84a3c11778abe692eaec637e39e85fac2896368bca4d3854defe84528f14
