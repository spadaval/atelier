---
created_at: "2026-11-10T16:50:56.902640945+00:00"
id: "atelier-c0i5"
evidence_type: "test"
commit: "74993259cc8e7b2dec7a7ccb9dd5632c948825db"
result: "pass"
captured_at: "2026-11-10T16:50:55.837384863+00:00"
command: "bash -c 'cd next && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp/5cuc cargo nextest run --build-jobs 4 -j 4 --locked 2>&1 | tail -2'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-5cuc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5cuc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'cd next && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp/5cuc cargo nextest run --build-jobs 4 -j 4 --locked 2>&1 | tail -2'"
updated_at: "2026-11-10T16:50:57.044058683+00:00"
---

## Summary

bash -c 'cd next && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp/5cuc cargo nextest run --build-jobs 4 -j 4 --locked 2>&1 | tail -2'

## Command

```console
bash -c 'cd next && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp/5cuc cargo nextest run --build-jobs 4 -j 4 --locked 2>&1 | tail -2'
```

Exit status: 0

## Stdout

Bytes: 97
SHA-256: 11c9c10e3289add98aa4c82eb040587d4d91f47669f73187c3ee655642a91f39
Test summary: Summary [   0.719s] 78 tests run: 78 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
