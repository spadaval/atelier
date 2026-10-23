---
created_at: "2026-10-23T15:47:50.188690192+00:00"
id: "atelier-e2ve"
evidence_type: "test"
commit: "e8805f7963b7c597f8009ce572eeaa1f5537b9ae"
result: "pass"
captured_at: "2026-10-23T15:47:47.833290308+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(disposition_of_an_approval_says_it_is_not_a_finding)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-jiz0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jiz0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "disposition of an approval ID says it is an approval, not a finding, with atelier review list <owner>"
updated_at: "2026-10-23T15:47:50.188991694+00:00"
---

## Summary

disposition of an approval ID says it is an approval, not a finding, with atelier review list <owner>

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(disposition_of_an_approval_says_it_is_not_a_finding)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 556
SHA-256: 641913689c143864c86dc40daadebadce348b95518405ac8876cd59cef310658
Test summary: Summary [   0.124s] 1 test run: 1 passed, 715 skipped
