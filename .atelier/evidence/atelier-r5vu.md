---
created_at: "2026-07-06T18:32:14.711637911+00:00"
id: "atelier-r5vu"
evidence_type: "validation"
captured_at: "2026-07-06T18:32:11.236914027+00:00"
command: "bash -lc 'tmp=$(mktemp -d); HOME=\"$tmp\" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -p atelier-cli commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret provider_review_open_action_reads_workflow_config_and_global_secret'"
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
title: "Controlled rerun for known HOME-sensitive provider token/fixture tests using an isolated HOME while preserving Cargo/Rustup toolchains."
updated_at: "2026-07-06T18:32:18.682325229+00:00"
---

## Summary

Controlled rerun for known HOME-sensitive provider token/fixture tests using an isolated HOME while preserving Cargo/Rustup toolchains.

## Command

```console
bash -lc 'tmp=$(mktemp -d); HOME="$tmp" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -p atelier-cli commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret provider_review_open_action_reads_workflow_config_and_global_secret'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 715
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.27s
────────────
 Nextest run ID 1dcdc3a6-02c2-4fba-91cd-8dc34a728e51 with nextest profile: default
    Starting 2 tests across 4 binaries (446 tests skipped)
        PASS [   0.011s] (1/2) atelier-cli commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_global_secret
        PASS [   0.727s] (2/2) atelier-cli::cli_integration provider_review_open_action_reads_workflow_config_and_global_secret
────────────
     Summary [   0.728s] 2 tests run: 2 passed, 446 skipped
```
