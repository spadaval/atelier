---
created_at: "2026-10-22T22:40:57.340498738+00:00"
id: "atelier-buxj"
evidence_type: "test"
commit: "1e5098895b49dc7f65e74a604b03b6da9bdaa57a"
result: "pass"
captured_at: "2026-10-22T22:40:19.796910070+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session a_close_after_approval_is_covered_by_merge_and_land_alike"
exit_status: "0"
agent_identity: "claude-implementer-r28p"
target:
  kind: "issue"
  id: "atelier-iaas"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-iaas"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Under both merge strategies: a follow-up filed and later linked to its mission, or a base-filed issue merged in and rewritten after the approval, keeps review.current and lands; closing either fails claims.reviewed and land"
updated_at: "2026-10-22T22:40:57.341229369+00:00"
---

## Summary

Under both merge strategies: a follow-up filed and later linked to its mission, or a base-filed issue merged in and rewritten after the approval, keeps review.current and lands; closing either fails claims.reviewed and land

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session a_close_after_approval_is_covered_by_merge_and_land_alike
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1124
Truncated: no

```text
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.93s
────────────
 Nextest run ID 736e306e-5534-44df-976f-fbed57a48174 with nextest profile: default
    Starting 1 test across 1 binary (104 tests skipped)
        SLOW [> 10.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
        SLOW [> 20.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
        SLOW [> 30.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
        PASS [  33.236s] (1/1) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
────────────
     Summary [  33.236s] 1 test run: 1 passed (1 slow), 104 skipped
        SLOW [  33.236s] (1/1) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
```

