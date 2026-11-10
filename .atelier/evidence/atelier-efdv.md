---
created_at: "2026-11-10T16:20:24.976253239+00:00"
id: "atelier-efdv"
evidence_type: "test"
commit: "700a496dfda97957c0be76e224b3d0fe33ed4377"
result: "pass"
captured_at: "2026-11-10T16:20:24.123450353+00:00"
command: "bash -c 'set -o pipefail; cd next && cargo fmt -- --check && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp/engh cargo nextest run --offline --build-jobs 4 --test-threads 4 2>&1 | tail -3'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-engh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-engh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check and the full suite (64 tests) after the round-one fixes"
updated_at: "2026-11-10T16:20:25.122410330+00:00"
---

## Summary

next/: cargo fmt --check and the full suite (64 tests) after the round-one fixes

## Command

```console
bash -c 'set -o pipefail; cd next && cargo fmt -- --check && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp/engh cargo nextest run --offline --build-jobs 4 --test-threads 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 210
SHA-256: 136127685e6cb4ab4e4720baae83994bf49780e222fcc853014c3156a8104c2a
Test summary: Summary [   0.425s] 64 tests run: 64 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
