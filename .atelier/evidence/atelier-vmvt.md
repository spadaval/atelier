---
created_at: "2026-10-22T21:04:36.562905429+00:00"
id: "atelier-vmvt"
evidence_type: "test"
commit: "047155d4993ffeebd8c2b10a20a7188990e9b771"
result: "pass"
captured_at: "2026-10-22T21:04:33.542547826+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session a_close_after_approval_is_covered_by_merge_and_land_alike base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions"
exit_status: "0"
agent_identity: "claude-implementer-n642-uw4x"
target:
  kind: "issue"
  id: "atelier-uw4x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uw4x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A close-only commit after approval (owner, and children with owner) passes merge --check, merge and land on the base; a close that also edits code or the Goal fails merge's review.current and land's land_base_done_claim"
updated_at: "2026-10-22T21:04:36.563414618+00:00"
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

Bytes: 683
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-1ztv/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.21s
────────────
 Nextest run ID a8bf2b9b-1bcd-4c96-b103-eb6ce4a557bf with nextest profile: default
    Starting 2 tests across 1 binary (90 tests skipped)
        PASS [   1.202s] (1/2) atelier-cli::session base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions
        PASS [   1.500s] (2/2) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
────────────
     Summary [   1.501s] 2 tests run: 2 passed, 90 skipped
```

