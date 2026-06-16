---
created_at: "2026-06-16T17:41:43.177338828+00:00"
id: "atelier-8jt5"
evidence_type: "test"
captured_at: "2026-06-16T17:41:37.148760681+00:00"
command: "bash -lc 'cargo test -p atelier-cli --test smoke_tests test_canonical_export_check_cli -- --nocapture && cargo test -p atelier-cli --test cli_integration test_top_level_help_only_shows_core_commands -- --nocapture && cargo test -p atelier-cli --test cli_integration test_root_status_summarizes_checkout_orientation -- --nocapture && cargo test -p atelier-app export:: -- --nocapture'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vuqb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vuqb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "focused command-surface and export tests pass"
updated_at: "2026-06-16T17:41:48.046189474+00:00"
---

## Summary

focused command-surface and export tests pass

## Command

```console
bash -lc 'cargo test -p atelier-cli --test smoke_tests test_canonical_export_check_cli -- --nocapture && cargo test -p atelier-cli --test cli_integration test_top_level_help_only_shows_core_commands -- --nocapture && cargo test -p atelier-cli --test cli_integration test_root_status_summarizes_checkout_orientation -- --nocapture && cargo test -p atelier-app export:: -- --nocapture'
```

Exit status: 0

## Stdout

Bytes: 1447
Truncated: no

```text

running 1 test
test smoke::cli_data::test_canonical_export_check_cli ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 51 filtered out; finished in 0.20s


running 1 test
test setup_guidance::test_top_level_help_only_shows_core_commands ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.01s


running 1 test
test setup_guidance::test_root_status_summarizes_checkout_orientation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.24s


running 10 tests
test export::tests::test_canonical_noop_export_is_deterministic ... ok
test export::tests::test_canonical_check_reports_invalid_duplicate_id ... ok
test export::tests::test_canonical_issue_type_is_explicit_not_label_derived ... ok
test export::tests::test_canonical_check_reports_stale_projection_metadata ... ok
test export::tests::test_canonical_export_removes_stale_record_file ... ok
test export::tests::test_canonical_changed_record_export_rewrites_issue ... ok
test export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift ... ok
test export::tests::test_canonical_markdown_serialization_stability ... ok
test export::tests::test_canonical_check_reports_dangling_link ... ok
test export::tests::test_canonical_export_preserves_issue_activity_sidecars ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 34 filtered out; finished in 0.13s
```

## Stderr

Bytes: 841
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.64s
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.65s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.67s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-b996d9a0f399a6f5)
```
