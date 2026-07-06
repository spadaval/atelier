---
created_at: "2026-06-25T00:54:32.451410184+00:00"
id: "atelier-fedc"
evidence_type: "test"
captured_at: "2026-06-25T00:54:27.837214078+00:00"
command: "cargo test -p atelier-cli --test cli_integration records_evidence::test_evidence_list_bounds_default_output -- --exact"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-5km8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5km8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "evidence list bounded ordering regression"
updated_at: "2026-06-25T00:54:35.663903890+00:00"
---

## Summary

evidence list bounded ordering regression

## Command

```console
cargo test -p atelier-cli --test cli_integration records_evidence::test_evidence_list_bounds_default_output -- --exact
```

Exit status: 0

## Stdout

Bytes: 185
Truncated: no

```text

running 1 test
test records_evidence::test_evidence_list_bounds_default_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 288 filtered out; finished in 2.58s
```

## Stderr

Bytes: 231
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.96s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```

