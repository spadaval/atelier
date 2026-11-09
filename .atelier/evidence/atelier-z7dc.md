---
created_at: "2026-11-09T20:23:59.476690263+00:00"
id: "atelier-z7dc"
evidence_type: "test"
commit: "132006b728303cb90ff8815b499dd4388966856b"
result: "pass"
captured_at: "2026-11-09T20:23:55.161625668+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test serve -p atelier-site"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ahpf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ahpf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "serve and atelier-site tests pass"
updated_at: "2026-11-09T20:23:59.622909300+00:00"
---

## Summary

serve and atelier-site tests pass

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test serve -p atelier-site
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 3580
SHA-256: 25983237d8dfc02a53a6916a1647b42698eef5753dc2f16da4ad0bfe6bdeb61a
Test summary: Summary [   2.180s] 25 tests run: 25 passed, 1 skipped
