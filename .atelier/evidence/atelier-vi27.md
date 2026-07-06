---
created_at: "2026-07-06T18:44:39.881454948+00:00"
id: "atelier-vi27"
evidence_type: "test"
captured_at: "2026-07-06T18:44:36.535506449+00:00"
command: "env HOME=/tmp/atelier-vqhi-review-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -E 'test(provider_request_review_pushes_source_before_opening_pr) or test(room_merge_requires_current_approval_and_resolved_blocking_findings)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Review provider approval and native finding resolution/merge safety"
updated_at: "2026-07-06T18:44:45.772685327+00:00"
---

## Summary

Review provider approval and native finding resolution/merge safety

## Command

```console
env HOME=/tmp/atelier-vqhi-review-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -E 'test(provider_request_review_pushes_source_before_opening_pr) or test(room_merge_requires_current_approval_and_resolved_blocking_findings)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 685
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.87s
────────────
 Nextest run ID bfa73fb4-002d-451b-ac1a-767f506cf992 with nextest profile: default
    Starting 2 tests across 9 binaries (715 tests skipped)
        PASS [   0.890s] (1/2) atelier-app review_room::tests::room_merge_requires_current_approval_and_resolved_blocking_findings
        PASS [   1.169s] (2/2) atelier-cli::cli_integration provider_request_review_pushes_source_before_opening_pr
────────────
     Summary [   1.170s] 2 tests run: 2 passed, 715 skipped
```

