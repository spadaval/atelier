---
created_at: "2026-10-23T14:33:31.862051354+00:00"
id: "atelier-y24q"
evidence_type: "test"
commit: "e7e76fde3096864c0735f705a8c742f5de060599"
result: "pass"
captured_at: "2026-10-23T14:33:11.520956860+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks invalid_commits_and_numbers_use_the_shared_invalid_value_form"
exit_status: "0"
agent_identity: "claude-implementer-phfo"
target:
  kind: "issue"
  id: "atelier-d8ku"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-d8ku"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The --commit, --resolution-commit, --retention-days and --limit flags use the shared invalid-value form with exit 2; an unknown full commit names the commit and git log as recovery, exit 1"
updated_at: "2026-10-23T14:33:31.862337337+00:00"
---

## Summary

The --commit, --resolution-commit, --retention-days and --limit flags use the shared invalid-value form with exit 2; an unknown full commit names the commit and git log as recovery, exit 1

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks invalid_commits_and_numbers_use_the_shared_invalid_value_form
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 645
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-phfo/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.89s
────────────
 Nextest run ID b2a51be6-c675-414a-baf7-de03451f9029 with nextest profile: default
    Starting 1 test across 10 binaries (899 tests skipped)
        PASS [   0.118s] (1/1) atelier-cli::cli_integration review_views::invalid_commits_and_numbers_use_the_shared_invalid_value_form
────────────
     Summary [   0.119s] 1 test run: 1 passed, 899 skipped
```

