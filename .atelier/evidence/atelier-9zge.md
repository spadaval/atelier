---
created_at: "2026-10-23T15:53:59.016255795+00:00"
id: "atelier-9zge"
evidence_type: "test"
commit: "fd8ad78e1bfc812396803e7a1f2e6a40784cc95a"
result: "pass"
captured_at: "2026-10-23T15:53:56.758459353+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_and_merge_check_report_an_invalid_policy_once) | test(merge_refusals_and_invalid_policy_carry_one_code_and_one_step)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "an invalid policy is reported once by land and merge --check, with one code and one Next (rerun after merging master)"
updated_at: "2026-10-23T15:53:59.016976862+00:00"
---

## Summary

an invalid policy is reported once by land and merge --check, with one code and one Next (rerun after merging master)

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
SHA-256: 6ffaca852777feace55eff39cfda5692e1412bdad88819b61584db259fd4e51e
Test summary: Summary [   0.265s] 2 tests run: 2 passed, 720 skipped
