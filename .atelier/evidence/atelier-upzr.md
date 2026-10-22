---
created_at: "2026-10-22T21:46:49.996246502+00:00"
id: "atelier-upzr"
evidence_type: "test"
commit: "aecb73084c8af59cdf31bfa9229dd7d55fd12db9"
result: "pass"
captured_at: "2026-10-22T21:46:47.117225502+00:00"
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
title: "record_commit tests incl. stale group, unstaged edit and rejecting hook"
updated_at: "2026-10-22T21:46:49.997283916+00:00"
---

## Summary

record_commit tests incl. stale group, unstaged edit and rejecting hook

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

Bytes: 1398
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-zuow/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.97s
────────────
 Nextest run ID 823bd754-9964-4b6d-acd2-5092d8a9e225 with nextest profile: default
    Starting 7 tests across 10 binaries (824 tests skipped)
        PASS [   0.110s] (1/7) atelier-cli::cli_integration record_commit::a_staged_change_that_is_not_a_record_refuses_before_anything_is_written
        PASS [   0.132s] (2/7) atelier-cli::cli_integration record_commit::a_hand_commit_ends_the_staged_group_and_unstaged_edits_are_not_swept_in
        PASS [   0.180s] (3/7) atelier-cli::cli_integration record_commit::a_rejecting_hook_leaves_the_group_for_the_next_commit
        PASS [   0.231s] (4/7) atelier-cli::cli_integration record_commit::a_review_step_and_the_close_each_commit_their_own_records
        PASS [   0.131s] (5/7) atelier-cli::cli_integration record_commit::an_unstaged_edit_to_a_staged_group_record_refuses
        PASS [   0.127s] (6/7) atelier-cli::cli_integration record_commit::commit_records_commits_only_the_records_the_command_wrote
        PASS [   0.171s] (7/7) atelier-cli::cli_integration record_commit::staged_records_are_committed_with_the_next_commit_records
────────────
     Summary [   0.353s] 7 tests run: 7 passed, 824 skipped
```

