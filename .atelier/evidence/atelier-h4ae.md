---
created_at: "2026-11-10T15:38:11.882297120+00:00"
id: "atelier-h4ae"
evidence_type: "validation"
commit: "41ec686905cc73fbd1600370bd65f95960843860"
result: "pass"
captured_at: "2026-11-10T15:37:59.000906414+00:00"
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
updated_at: "2026-11-10T15:38:12.020860318+00:00"
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
