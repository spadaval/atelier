---
created_at: "2026-11-04T23:33:01.081189699+00:00"
id: "atelier-y4a6"
evidence_type: "test"
commit: "2b5ff696abb8b3f385d712d5512dbdac451a020c"
result: "pass"
captured_at: "2026-11-04T23:32:58.776749265+00:00"
command: "cargo nextest run -p atelier-cli brief_shows_the_strategy_outcome"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-0a52"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0a52"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Both brief forms show the strategy's Outcome, and the mission's when the strategy has none or there is no strategy"
updated_at: "2026-11-04T23:33:01.081495220+00:00"
---

## Summary

Both brief forms show the strategy's Outcome, and the mission's when the strategy has none or there is no strategy

## Command

```console
cargo nextest run -p atelier-cli brief_shows_the_strategy_outcome
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 580
SHA-256: f3d1f1eb35e56da680bc7d6d4775fb92138cc7698e0263e250662e4d41b8ee10
Test summary: Summary [   0.243s] 1 test run: 1 passed, 741 skipped
