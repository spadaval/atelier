---
created_at: "2026-11-09T20:14:44.651739190+00:00"
id: "atelier-zsmu"
evidence_type: "test"
commit: "7ee930481258b37712b84d1a973a087a5abdb2ee"
result: "pass"
captured_at: "2026-11-09T20:13:34.931794647+00:00"
command: "cargo nextest run -j 4 -p atelier-cli"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-59fz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-59fz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run -j 4 -p atelier-cli"
updated_at: "2026-11-09T20:14:44.789526354+00:00"
---

## Summary

cargo nextest run -j 4 -p atelier-cli

## Command

```console
cargo nextest run -j 4 -p atelier-cli
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 111813
SHA-256: 688a6760cddc707cc4438e4a2dc2b883588d6aa23273c84216e2d306ead92779
Test summary: Summary [  65.587s] 884 tests run: 884 passed, 1 skipped
