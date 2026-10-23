---
created_at: "2026-10-23T15:47:47.729091953+00:00"
id: "atelier-lunx"
evidence_type: "test"
commit: "e8805f7963b7c597f8009ce572eeaa1f5537b9ae"
result: "pass"
captured_at: "2026-10-23T15:47:45.291299027+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_and_merge_check_report_an_invalid_policy_once) | test(merge_refusals_and_invalid_policy_carry_one_code_and_one_step)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-y0en"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-y0en"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "an invalid policy is reported once by land and merge --check, with one code and one Next"
updated_at: "2026-10-23T15:47:47.729396362+00:00"
---

## Summary

an invalid policy is reported once by land and merge --check, with one code and one Next

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_and_merge_check_report_an_invalid_policy_once) | test(merge_refusals_and_invalid_policy_carry_one_code_and_one_step)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 689
SHA-256: 7aa5a68a44f16afff77fd681260e6f3e74a6e163ecc7a7c305c436a64df57b38
Test summary: Summary [   0.259s] 2 tests run: 2 passed, 714 skipped
