---
created_at: "2026-07-06T17:58:42.121611038+00:00"
id: "atelier-iwcj"
evidence_type: "test"
captured_at: "2026-07-06T17:58:37.770454550+00:00"
command: "cargo nextest run -p atelier-app -p atelier-cli pr::tests::derived_open_context_uses_the_branch_owner_and_workflow_branches commands::workflow::tests::review_open_action_persists_room_review_field review_surface_derives_open_context_and_uses_submit_and_show"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-odwi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-odwi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Passed derived review-open behavior for direct and workflow paths, including owner/title/body/branch derivation."
updated_at: "2026-07-06T17:58:46.596411878+00:00"
---

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
