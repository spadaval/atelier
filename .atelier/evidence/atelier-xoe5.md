---
created_at: "2026-11-05T01:46:37.022895571+00:00"
id: "atelier-xoe5"
evidence_type: "test"
commit: "529a6ce232ba30c4bc1585aaf7c984982b133bf3"
result: "pass"
captured_at: "2026-11-05T01:46:32.780094862+00:00"
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
updated_at: "2026-11-05T01:46:37.023540404+00:00"
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
SHA-256: 3f93f5ac9c5df5aba7f8944dc6a2da1f97dc1d35e465aa014b324f4eab8ebc54
Test summary: Summary [   0.369s] 1 test run: 1 passed, 750 skipped
