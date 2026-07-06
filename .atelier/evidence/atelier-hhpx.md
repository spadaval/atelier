---
created_at: "2026-07-06T17:59:57.887211694+00:00"
id: "atelier-hhpx"
evidence_type: "test"
captured_at: "2026-07-06T17:59:54.647890218+00:00"
command: "cargo nextest run -E 'test(test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs) or test(test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence) or test(test_history_rejects_removed_query_and_objective_scope_flags) or test(test_history_empty_states_and_invalid_limit)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ll1n"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ll1n"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "History complexity budget: repository and single-issue reads stay bounded; removed query/objective flags reject; quiet output stays terse"
updated_at: "2026-07-06T18:00:02.105414513+00:00"
---

## Summary

History complexity budget: repository and single-issue reads stay bounded; removed query/objective flags reject; quiet output stays terse

## Command

```console
cargo nextest run -E 'test(test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs) or test(test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence) or test(test_history_rejects_removed_query_and_objective_scope_flags) or test(test_history_empty_states_and_invalid_limit)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 956
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-yysm/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.22s
────────────
 Nextest run ID c1378c1f-12db-40e7-8185-83e9dffc1d9a with nextest profile: default
    Starting 4 tests across 9 binaries (717 tests skipped)
        PASS [   0.180s] (1/4) atelier-cli::cli_integration issues::test_history_rejects_removed_query_and_objective_scope_flags
        PASS [   0.327s] (2/4) atelier-cli::cli_integration issues::test_history_empty_states_and_invalid_limit
        PASS [   0.440s] (3/4) atelier-cli::cli_integration issues::test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs
        PASS [   0.713s] (4/4) atelier-cli::cli_integration issues::test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence
────────────
     Summary [   0.714s] 4 tests run: 4 passed, 717 skipped
```

