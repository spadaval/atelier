---
created_at: "2026-06-16T19:24:49.123948747+00:00"
id: "atelier-lxod"
evidence_type: "validation"
captured_at: "2026-06-16T19:24:40.282686389+00:00"
command: "bash -lc '! rg -n -f /tmp/atelier-evidence-noise-patterns .atelier/evidence crates/atelier-records/src; cargo fmt -- --check; cargo test -p atelier-records evidence -- --nocapture; cargo test -p atelier-cli test_evidence_capture_records_command_metadata_and_attaches_targets -- --nocapture; cargo test -p atelier-cli test_evidence_capture_records_nonzero_exit_as_command_metadata -- --nocapture; target/debug/atelier lint atelier-yn3u; target/debug/atelier export --check; git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yn3u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yn3u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Final evidence shape and tests pass"
updated_at: "2026-06-16T19:24:52.573709588+00:00"
---

## Summary

Final evidence shape and tests pass

## Command

```console
bash -lc '! rg -n -f /tmp/atelier-evidence-noise-patterns .atelier/evidence crates/atelier-records/src; cargo fmt -- --check; cargo test -p atelier-records evidence -- --nocapture; cargo test -p atelier-cli test_evidence_capture_records_command_metadata_and_attaches_targets -- --nocapture; cargo test -p atelier-cli test_evidence_capture_records_nonzero_exit_as_command_metadata -- --nocapture; target/debug/atelier lint atelier-yn3u; target/debug/atelier export --check; git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1621
Truncated: no

```text

running 4 tests
test tests::mission_render_normalizes_legacy_evidence_attachments ... ok
test tests::issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter ... ok
test tests::evidence_record_renders_and_parses_deterministically_without_data_blob ... ok
test tests::legacy_evidence_data_record_loads_into_typed_front_matter ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 39 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 2.03s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test records_evidence::test_evidence_capture_records_nonzero_exit_as_command_metadata ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 0.22s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1106
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-6e5299045d686aef)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.82s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.74s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
