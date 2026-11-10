---
created_at: "2026-11-10T16:12:33.863740522+00:00"
id: "atelier-jbaq"
evidence_type: "test"
commit: "6531002d072d4e640125cabbff0575d68dd907d4"
result: "pass"
captured_at: "2026-11-10T16:12:33.075598333+00:00"
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
title: "next/: cargo fmt --check and the full suite (61 tests), at the commit under review"
updated_at: "2026-11-10T16:12:34.005561161+00:00"
---

## Summary

next/: cargo fmt --check and the full suite (61 tests), at the commit under review

## Command

```console
bash -c 'set -o pipefail; cd next && cargo fmt -- --check && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp/engh cargo nextest run --offline --build-jobs 4 --test-threads 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 210
SHA-256: 22ec9a28aee684b589e178fd554e470750a703ea00bc167fb3939b79b75e43de
Test summary: Summary [   0.369s] 61 tests run: 61 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
