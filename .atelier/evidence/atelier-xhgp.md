---
created_at: "2026-10-22T22:18:55.033617838+00:00"
id: "atelier-xhgp"
evidence_type: "test"
commit: "bbd0f9964dda4f9fd3684a30eab9e31bc6404c0e"
result: "pass"
captured_at: "2026-10-22T22:18:39.276178332+00:00"
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
title: "A follow-up filed after approval with --discovered-from and linked to its mission in a later commit keeps review.current and lands under both merge strategies; closing it still fails claims.reviewed and land"
updated_at: "2026-10-22T22:18:55.033912156+00:00"
---

## Summary

A follow-up filed after approval with --discovered-from and linked to its mission in a later commit keeps review.current and lands under both merge strategies; closing it still fails claims.reviewed and land

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.21s
────────────
 Nextest run ID a9df6f0f-af8b-4214-b4c5-18fbeb50352f with nextest profile: default
    Starting 1 test across 1 binary (102 tests skipped)
        SLOW [> 10.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
        PASS [  14.309s] (1/1) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
────────────
     Summary [  14.309s] 1 test run: 1 passed (1 slow), 102 skipped
        SLOW [  14.309s] (1/1) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
```

