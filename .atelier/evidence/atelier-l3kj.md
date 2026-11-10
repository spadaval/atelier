---
created_at: "2026-11-10T14:48:59.367275389+00:00"
id: "atelier-l3kj"
evidence_type: "validation"
commit: "715d9637a990921a44b3fd7eba8f07d3d8a7ac78"
result: "pass"
captured_at: "2026-11-10T14:48:46.729994968+00:00"
command: "nice -n 19 ionice -c3 python3 scripts/scorecard/freeze.py verify"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-5fsj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5fsj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "the snapshot still matches its manifest after the runs"
updated_at: "2026-11-10T14:48:59.513758016+00:00"
---

## Summary

the snapshot still matches its manifest after the runs

## Command

```console
nice -n 19 ionice -c3 python3 scripts/scorecard/freeze.py verify
```

Exit status: 0

## Stdout

Bytes: 56
SHA-256: 55e4aea1ff1f1811b736e652f30dd98e1a1505b075a0133c1f85137cb45ab2d3

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
