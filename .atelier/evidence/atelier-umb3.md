---
created_at: "2026-11-10T16:55:45.989486651+00:00"
id: "atelier-umb3"
evidence_type: "test"
commit: "85abd947e7cc8402d57da8168b891a02e9f5853b"
result: "pass"
captured_at: "2026-11-10T16:55:44.881520492+00:00"
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
updated_at: "2026-11-10T16:55:46.135888618+00:00"
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
SHA-256: 4fc74616020055a359bff4e88f42e19456e586f6ce3d189fa9ce1907e0884dc3
Test summary: Summary [   0.739s] 78 tests run: 78 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
