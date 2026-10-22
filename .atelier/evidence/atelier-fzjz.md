---
created_at: "2026-10-22T21:14:44.992822247+00:00"
id: "atelier-fzjz"
evidence_type: "test"
commit: "be10bc47ae8604e300b20d1e4fc335e82f789588"
result: "pass"
captured_at: "2026-10-22T21:14:38.392675129+00:00"
command: "cargo nextest run -j 4 -E 'test(record_commit)'"
exit_status: "0"
agent_identity: "claude-implementer-4ixu"
target:
  kind: "issue"
  id: "atelier-4ixu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4ixu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "record_commit CLI tests: commit only own records, refuse foreign staged changes, group staged records"
updated_at: "2026-10-22T21:14:44.993573146+00:00"
---

## Summary

record_commit CLI tests: commit only own records, refuse foreign staged changes, group staged records

## Command

```console
cargo nextest run -j 4 -E 'test(record_commit)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1057
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-zuow/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.94s
────────────
 Nextest run ID dcf64743-1fff-48a3-b6d9-7a9f53e1b094 with nextest profile: default
    Starting 4 tests across 10 binaries (812 tests skipped)
        PASS [   0.080s] (1/4) atelier-cli::cli_integration record_commit::a_staged_change_that_is_not_a_record_refuses_before_anything_is_written
        PASS [   0.083s] (2/4) atelier-cli::cli_integration record_commit::commit_records_commits_only_the_records_the_command_wrote
        PASS [   0.133s] (3/4) atelier-cli::cli_integration record_commit::staged_records_are_committed_with_the_next_commit_records
        PASS [   0.144s] (4/4) atelier-cli::cli_integration record_commit::a_review_step_and_the_close_each_commit_their_own_records
────────────
     Summary [   0.146s] 4 tests run: 4 passed, 812 skipped
```

