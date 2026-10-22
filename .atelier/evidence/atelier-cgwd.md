---
created_at: "2026-10-22T21:04:33.422526872+00:00"
id: "atelier-cgwd"
evidence_type: "test"
commit: "047155d4993ffeebd8c2b10a20a7188990e9b771"
result: "pass"
captured_at: "2026-10-22T21:04:23.761238385+00:00"
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
title: "A close-only commit after approval (owner, and children with owner) passes merge --check, merge and land on the base; a close that also edits code or the Goal fails merge's review.current and land's land_base_done_claim"
updated_at: "2026-10-22T21:04:33.423269145+00:00"
---

## Summary

A close-only commit after approval (owner, and children with owner) passes merge --check, merge and land on the base; a close that also edits code or the Goal fails merge's review.current and land's land_base_done_claim

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

Bytes: 743
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-1ztv/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.49s
────────────
 Nextest run ID 52a4a00d-dec7-479a-8ddc-a7a0c7d066fd with nextest profile: default
    Starting 2 tests across 1 binary (90 tests skipped)
        PASS [   1.446s] (1/2) atelier-cli::session base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions
        PASS [   1.616s] (2/2) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
────────────
     Summary [   1.616s] 2 tests run: 2 passed, 90 skipped
```

