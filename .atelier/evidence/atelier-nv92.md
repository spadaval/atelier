---
created_at: "2026-07-06T19:30:23.930745207+00:00"
id: "atelier-nv92"
evidence_type: "test"
captured_at: "2026-07-06T19:30:21.162852153+00:00"
command: "cargo nextest run -p atelier-cli --test cli_integration test_man_"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-u4gx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u4gx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli --test cli_integration test_man_"
updated_at: "2026-07-06T19:30:27.693990582+00:00"
---

## Summary

cargo nextest run -p atelier-cli --test cli_integration test_man_

## Command

```console
cargo nextest run -p atelier-cli --test cli_integration test_man_
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1052
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.00s
────────────
 Nextest run ID a23575a0-84a4-4aac-a5c6-01ac1325da54 with nextest profile: default
    Starting 5 tests across 1 binary (262 tests skipped)
        PASS [   0.015s] (1/5) atelier-cli::cli_integration setup_guidance::test_man_work_model_explains_scope_and_runs_without_tracker_state
        PASS [   0.017s] (2/5) atelier-cli::cli_integration setup_guidance::test_man_lists_roles_and_topics
        PASS [   0.030s] (3/5) atelier-cli::cli_integration setup_guidance::test_man_rejects_unknown_pages_and_admin_degrades_before_init
        PASS [   0.125s] (4/5) atelier-cli::cli_integration setup_guidance::test_man_worker_guides_empty_checkout_without_repeating_status
        PASS [   0.471s] (5/5) atelier-cli::cli_integration setup_guidance::test_man_worker_names_current_work
────────────
     Summary [   0.471s] 5 tests run: 5 passed, 262 skipped
```
