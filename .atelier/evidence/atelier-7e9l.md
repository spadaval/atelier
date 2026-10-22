---
created_at: "2026-10-22T22:24:30.975226398+00:00"
id: "atelier-7e9l"
evidence_type: "test"
commit: "52539c60654fd8ff29bbb07a5e79fa400fcbc492"
result: "pass"
captured_at: "2026-10-22T22:24:21.696416777+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session -E 'test(waiting_branches_show) | test(parent_owned_branch) | test(done_claims_on_work_branches) | test(ready_work_in_flight)'"
exit_status: "0"
agent_identity: "claude-implementer-tciw"
target:
  kind: "issue"
  id: "atelier-tciw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tciw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Branches rows show readiness (clean, conflicted, stale approval, not approved, shared files, unchanged Git state) and a parent-owned branch's children activity; existing branch-row tests pass"
updated_at: "2026-10-22T22:24:30.975624228+00:00"
---

## Summary

Branches rows show readiness (clean, conflicted, stale approval, not approved, shared files, unchanged Git state) and a parent-owned branch's children activity; existing branch-row tests pass

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session -E 'test(waiting_branches_show) | test(parent_owned_branch) | test(done_claims_on_work_branches) | test(ready_work_in_flight)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 945
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-tciw/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.21s
────────────
 Nextest run ID 062ef5f2-cff5-40b8-9f36-ab0b03a3bd22 with nextest profile: default
    Starting 4 tests across 1 binary (101 tests skipped)
        PASS [   0.216s] (1/4) atelier-cli::session a_parent_owned_branch_shows_its_childrens_activity
        PASS [   0.433s] (2/4) atelier-cli::session done_claims_on_work_branches_say_they_are_not_merged
        PASS [   0.585s] (3/4) atelier-cli::session ready_work_in_flight_on_another_branch_is_marked_with_its_state_there
        PASS [   0.751s] (4/4) atelier-cli::session waiting_branches_show_merge_readiness_against_the_base
────────────
     Summary [   0.751s] 4 tests run: 4 passed, 101 skipped
```

