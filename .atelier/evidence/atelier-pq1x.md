---
created_at: "2026-11-10T05:19:26.143038164+00:00"
id: "atelier-pq1x"
evidence_type: "validation"
commit: "2dfeef82d317b0bb97a63633bc74feb58157d392"
result: "pass"
captured_at: "2026-11-10T05:19:16.379379833+00:00"
command: "python3 scripts/scorecard/freeze.py verify"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-scx2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-scx2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "freeze.py verify: every snapshot file matches the manifest's size and SHA-256, every entry exists, all modes owner-only"
updated_at: "2026-11-10T05:19:26.292963577+00:00"
---

## Summary

freeze.py verify: every snapshot file matches the manifest's size and SHA-256, every entry exists, all modes owner-only

## Command

```console
python3 scripts/scorecard/freeze.py verify
```

Exit status: 0

## Stdout

Bytes: 56
SHA-256: 55e4aea1ff1f1811b736e652f30dd98e1a1505b075a0133c1f85137cb45ab2d3

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
