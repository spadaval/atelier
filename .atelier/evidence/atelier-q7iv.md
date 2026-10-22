---
created_at: "2026-10-22T22:51:34.227321259+00:00"
id: "atelier-q7iv"
evidence_type: "test"
commit: "75d2dc1389174d45d8362abf14b10dc0eef309a7"
result: "pass"
captured_at: "2026-10-22T22:51:12.921886062+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session a_close_after_approval_is_covered_by_merge_and_land_alike"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "## Summary\n\nUnder both merge strategies: a follow-up filed and later linked to its mission, or a base-filed issue merged in and rewritten after the approval, keeps review.current and lands; closing either fails claims.reviewed and land\n\n## Command\n\n```console\ncargo nextest run -j 4 -p atelier-cli --test session a_close_after_approval_is_covered_by_merge_and_land_alike\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nTruncated: no\n\n```text\n```\n\n## Stderr\n\nBytes: 1124\nTruncated: no\n\n```text\n   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-app)\n   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.93s\n────────────\n Nextest run ID 736e306e-5534-44df-976f-fbed57a48174 with nextest profile: default\n    Starting 1 test across 1 binary (104 tests skipped)\n        SLOW [> 10.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike\n        SLOW [> 20.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike\n        SLOW [> 30.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike\n        PASS [  33.236s] (1/1) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike\n────────────\n     Summary [  33.236s] 1 test run: 1 passed (1 slow), 104 skipped\n        SLOW [  33.236s] (1/1) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike\n```\n (rerun after merging master)"
updated_at: "2026-10-22T22:51:34.227788940+00:00"
---

## Summary

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
 (rerun after merging master)

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

Bytes: 791
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.80s
────────────
 Nextest run ID 33b3629d-f2e8-4204-a295-fa66097d7d81 with nextest profile: default
    Starting 1 test across 1 binary (106 tests skipped)
        SLOW [> 10.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
        PASS [  19.222s] (1/1) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
────────────
     Summary [  19.222s] 1 test run: 1 passed (1 slow), 106 skipped
        SLOW [  19.222s] (1/1) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
```

