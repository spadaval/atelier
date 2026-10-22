---
created_at: "2026-10-22T21:27:25.972303141+00:00"
id: "atelier-ue6j"
evidence_type: "test"
commit: "3f36a1461ce1a4c6c21744587c0fb23ed33d5c46"
result: "pass"
captured_at: "2026-10-22T21:27:08.817039216+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session a_close_after_approval_is_covered_by_merge_and_land_alike base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions"
exit_status: "0"
agent_identity: "claude-implementer-n642-uw4x"
target:
  kind: "issue"
  id: "atelier-n642"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n642"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Under fast_forward_only and merge_commit: close-only, child, and child+owner-in-one-commit closes after approval pass merge and land on the base; a close with code, Goal edit, note, sibling close, new done issue, self-approval or open FIX NOW fails merge before integrating and land_base_done_claim after a forced integration"
updated_at: "2026-10-22T21:27:25.972870399+00:00"
---

## Summary

Under fast_forward_only and merge_commit: close-only, child, and child+owner-in-one-commit closes after approval pass merge and land on the base; a close with code, Goal edit, note, sibling close, new done issue, self-approval or open FIX NOW fails merge before integrating and land_base_done_claim after a forced integration

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session a_close_after_approval_is_covered_by_merge_and_land_alike base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 978
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-1ztv/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.95s
────────────
 Nextest run ID 6976e0df-7ded-4cd4-bb53-2e4f320cb4e5 with nextest profile: default
    Starting 2 tests across 1 binary (90 tests skipped)
        PASS [   1.109s] (1/2) atelier-cli::session base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions
        SLOW [> 10.000s] (───) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
        PASS [  10.941s] (2/2) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
────────────
     Summary [  10.941s] 2 tests run: 2 passed (1 slow), 90 skipped
        SLOW [  10.941s] (2/2) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
```

