---
created_at: "2026-10-22T21:10:06.815350984+00:00"
id: "atelier-h6fp"
evidence_type: "test"
commit: "d8351d878a66a64355662ebeaa03603495bd8fa8"
result: "pass"
captured_at: "2026-10-22T21:09:58.639444591+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(land_push) | test(land_ends) | test(setup_is_atomic)'"
exit_status: "0"
agent_identity: "claude-implementer-ecah"
target:
  kind: "issue"
  id: "atelier-ecah"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ecah"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land --push review fixes: own-upstream guard, checked commit, verdict, no pipefail prefix"
updated_at: "2026-10-22T21:10:06.816191020+00:00"
---

## Summary

land --push review fixes: own-upstream guard, checked commit, verdict, no pipefail prefix

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(land_push) | test(land_ends) | test(setup_is_atomic)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1073
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-yim7/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.54s
────────────
 Nextest run ID a6d2616c-0ac7-4457-9bf2-002f5f343463 with nextest profile: default
    Starting 5 tests across 6 binaries (632 tests skipped)
        PASS [   0.221s] (1/5) atelier-cli::session land_push_pushes_nothing_when_a_check_moves_head_during_land
        PASS [   0.242s] (2/5) atelier-cli::session land_push_refuses_an_upstream_that_is_not_the_branchs_own_remote_branch
        PASS [   0.253s] (3/5) atelier-cli::session land_push_reports_a_failed_push_and_a_missing_upstream
        PASS [   0.284s] (4/5) atelier-cli::session land_ends_with_a_verdict_line_and_push_follows_only_a_pass
        PASS [   0.136s] (5/5) atelier-cli::session setup_is_atomic_idempotent_and_preserves_other_handlers
────────────
     Summary [   0.358s] 5 tests run: 5 passed, 632 skipped
```

