---
created_at: "2026-10-23T14:41:29.528781996+00:00"
id: "atelier-ao8h"
evidence_type: "test"
commit: "fa47f83a4dbb588e03d70cbfeed54763ddd47cbc"
result: "pass"
captured_at: "2026-10-23T14:41:26.712719038+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(approval_records_committed_by_author_remain_independent_and_merge_has_no_writes) | test(git_hooks_are_explicitly_disabled_in_the_final_integration_fence) | test(separately_checked_out_clean_base_is_fast_forwarded_and_source_branch_is_cleaned)'"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-2tux"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2tux"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "successful merge prints exactly the merged line with Next and merge: passed, no gate lines; hooks notice only with a repository hook; detached-checkout note kept"
updated_at: "2026-10-23T14:41:29.529092014+00:00"
---

## Summary

successful merge prints exactly the merged line with Next and merge: passed, no gate lines; hooks notice only with a repository hook; detached-checkout note kept

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(approval_records_committed_by_author_remain_independent_and_merge_has_no_writes) | test(git_hooks_are_explicitly_disabled_in_the_final_integration_fence) | test(separately_checked_out_clean_base_is_fast_forwarded_and_source_branch_is_cleaned)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 845
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.28s
────────────
 Nextest run ID 5cd5a95e-ba38-4ae3-b08b-3eea541f0c3d with nextest profile: default
    Starting 3 tests across 10 binaries (895 tests skipped)
        PASS [   0.209s] (1/3) atelier-cli::merge_gates git_hooks_are_explicitly_disabled_in_the_final_integration_fence
        PASS [   0.222s] (2/3) atelier-cli::merge_gates approval_records_committed_by_author_remain_independent_and_merge_has_no_writes
        PASS [   0.236s] (3/3) atelier-cli::merge_gates separately_checked_out_clean_base_is_fast_forwarded_and_source_branch_is_cleaned
────────────
     Summary [   0.236s] 3 tests run: 3 passed, 895 skipped
```

