---
created_at: "2026-10-22T23:44:59.155620717+00:00"
id: "atelier-3h82"
evidence_type: "test"
commit: "5959474657df56a8fd49bab0b7215c7fb6a05268"
result: "pass"
captured_at: "2026-10-22T23:44:57.205521458+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks --test session land_push"
exit_status: "0"
agent_identity: "claude-implementer-fngz"
target:
  kind: "issue"
  id: "atelier-976s"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-976s"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land --push refuses a base whose upstream is gone or a URL (nothing pushed, no pass line), still pushes work branches; the note names an unresolved upstream"
updated_at: "2026-10-22T23:44:59.155891161+00:00"
---

## Summary

land --push refuses a base whose upstream is gone or a URL (nothing pushed, no pass line), still pushes work branches; the note names an unresolved upstream

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks --test session land_push
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1038
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.21s
────────────
 Nextest run ID 70d4de9d-fc3e-4d99-9971-f9f77e218b9a with nextest profile: default
    Starting 5 tests across 1 binary (106 tests skipped)
        PASS [   0.204s] (1/5) atelier-cli::session land_push_pushes_nothing_when_a_check_moves_head_during_land
        PASS [   0.226s] (2/5) atelier-cli::session land_push_refuses_an_upstream_that_is_not_the_branchs_own_remote_branch
        PASS [   0.236s] (3/5) atelier-cli::session land_push_reports_a_failed_push_and_a_missing_upstream
        PASS [   0.259s] (4/5) atelier-cli::session land_push_refuses_pushing_a_base_whose_upstream_does_not_resolve
        PASS [   0.294s] (5/5) atelier-cli::session land_push_still_pushes_a_work_branch_when_the_base_upstream_does_not_resolve
────────────
     Summary [   0.499s] 5 tests run: 5 passed, 106 skipped
```

