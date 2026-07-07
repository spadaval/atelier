---
created_at: "2026-07-06T18:35:15.869350962+00:00"
id: "atelier-7ti6"
evidence_type: "validation"
captured_at: "2026-07-06T18:35:15.582143508+00:00"
command: "bash -lc 'set -euo pipefail; target/debug/atelier evidence show atelier-iwcj; target/debug/atelier history --issue atelier-ye11 --limit 5'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Integrated-surface smoke proof: child evidence and bounded epic history remain readable after review-surface integration; provider surface is covered separately by atelier-sdnv."
updated_at: "2026-07-06T18:35:19.730660015+00:00"
---

## Summary

Integrated-surface smoke proof: child evidence and bounded epic history remain readable after review-surface integration; provider surface is covered separately by atelier-sdnv.

## Command

```console
bash -lc 'set -euo pipefail; target/debug/atelier evidence show atelier-iwcj; target/debug/atelier history --issue atelier-ye11 --limit 5'
```

Exit status: 0

## Stdout

Bytes: 4707
Truncated: yes

```text
atelier-iwcj [evidence] recorded - Passed derived review-open behavior for direct and workflow paths, including owner/title/body/branch derivation.
===================================================================================================================================================
Status:      recorded
Kind:        test
Captured:    2026-07-06T17:58:37.770454550+00:00
Command:     cargo nextest run -p atelier-app -p atelier-cli pr::tests::derived_open_context_uses_the_branch_owner_and_workflow_branches commands::workflow::tests::review_open_action_persists_room_review_field review_surface_derives_open_context_and_uses_submit_and_show
Exit Status: 0
Target:      issue/atelier-odwi (validates)
Producer:    (none)
Path:        (none)
URI:         (none)
Created:     2026-07-06T17:58:42.121611038+00:00
Updated:     2026-07-06T17:58:46.596411878+00:00
Summary
-------
## Summary

Passed derived review-open behavior for direct and workflow paths, including owner/title/body/branch derivation.

## Command

```console
cargo nextest run -p atelier-app -p atelier-cli pr::tests::derived_open_context_uses_the_branch_owner_and_workflow_branches commands::workflow::tests::review_open_action_persists_room_review_field review_surface_derives_open_context_and_uses_submit_and_show
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 885
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.14s
────────────
 Nextest run ID 91eca454-89be-497e-817c-ef5a23d78069 with nextest profile: default
    Starting 3 tests across 5 binaries (555 tests skipped)
        PASS [   0.121s] (1/3) atelier-app pr::tests::derived_open_context_uses_the_branch_owner_and_workflow_branches
        PASS [   0.221s] (2/3) atelier-cli commands::workflow::tests::review_open_action_persists_room_review_field
        PASS [   0.900s] (3/3) atelier-cli::cli_integration review_surface_derives_open_context_and_uses_submit_and_show
────────────
     Summary [   0.902s] 3 tests run: 3 passed, 555 skipped
```
Output Summary
--------------
Stdout: 0 bytes, truncated: no
(none)
Stderr: 885 bytes, truncated: no
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.14s
────────────
 Nextest run ID 91eca454-89be-497e-817c-ef5a23d78069 with nextest profile: default
    Starting 3 tests across 5 binaries (555 tests skipped)
        PASS [   0.121s] (1/3) atelier-app pr::tests::derived_open_context_uses_the_branch_owner_and_workflow_branches
        PASS [   0.221s] (2/3) atelier-cli commands::workflow::tests::review_open_action_persists_room_review_field
        PASS [   0.900s] (3/3) atelier-cli::cli_integration review_surface_derives_open_context_and_uses_submit_and_show
────────────
     Summary [   0.902s] 3 tests run: 3 passed, 555 skipped
History
-------
Scope:          issue atelier-ye11 - Epic: Simplify review command surface
Source:         canonical .atelier issue activity, records, evidence, status roles, review artifacts, and record links; local runtime diagnostics excluded
Ordering:       newest first, timestamp then record/path
Limit:          5
Showing:        5 of 64 events

Events
------
  Epic: Simplify review command surface: Attached evidence atelier-su6w
    2026-07-06 14:35 -04:00 | evidence_attached | root | issue/atelier-ye11
  Epic: Simplify review command surface: Attached evidence atelier-zjst to issue/atelier-ye11 (validates)
    2026-07-06 14:35 -04:00 | evidence_attached | (system) | issue/atelier-ye11
  Epic: Simplify review command surface: Attached evidence atelier-xs5p to issue/atelier-ye11 (validates)
    2
```

## Stderr

Bytes: 0
Truncated: no

```text
```
