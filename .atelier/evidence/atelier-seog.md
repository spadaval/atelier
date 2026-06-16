---
created_at: "2026-06-15T17:25:59.475534921+00:00"
id: "atelier-seog"
evidence_type: "validation"
captured_at: "2026-06-15T17:25:57.909808504+00:00"
command: "sh -c 'cargo test -p atelier-core && RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets && rg \"pub struct Issue|pub struct DomainRecord|pub struct EvidenceRecordData\" crates/atelier-core/src/lib.rs && ! rg \"pub struct Issue|pub struct DomainRecord|pub struct EvidenceRecordData\" crates/atelier-cli/src/models.rs'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-nbni"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nbni"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Richer domain model structs now live in atelier-core; atelier-cli::models is a transitional re-export adapter, and workspace warning-as-error compile plus core tests pass."
updated_at: "2026-06-15T17:26:02.801576747+00:00"
---

## Summary

Richer domain model structs now live in atelier-core; atelier-cli::models is a transitional re-export adapter, and workspace warning-as-error compile plus core tests pass.

## Command

```console
sh -c 'cargo test -p atelier-core && RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets && rg "pub struct Issue|pub struct DomainRecord|pub struct EvidenceRecordData" crates/atelier-core/src/lib.rs && ! rg "pub struct Issue|pub struct DomainRecord|pub struct EvidenceRecordData" crates/atelier-cli/src/models.rs'
```

Exit status: 0

## Stdout

Bytes: 454
Truncated: no

```text

running 3 tests
test tests::record_id_rejects_empty_values ... ok
test tests::relation_requires_kind_and_role ... ok
test tests::record_id_preserves_valid_text ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

pub struct Issue {
pub struct DomainRecord {
pub struct EvidenceRecordData {
```

## Stderr

Bytes: 322
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_core-0c17e39e9d635446)
   Doc-tests atelier_core
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.26s
```
