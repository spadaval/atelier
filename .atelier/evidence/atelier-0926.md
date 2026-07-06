---
created_at: "2026-07-06T18:18:03.866974648+00:00"
id: "atelier-0926"
evidence_type: "test"
captured_at: "2026-07-06T18:17:17.838123492+00:00"
command: "sh -c 'tmp_home=$(mktemp -d); HOME=\"$tmp_home\" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run --status-level fail --final-status-level fail'"
exit_status: "0"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-eqq6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-eqq6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c 'tmp_home=$(mktemp -d); HOME=\"$tmp_home\" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run --status-level fail --final-status-level fail'"
updated_at: "2026-07-06T18:18:07.761715932+00:00"
---

## Summary

sh -c 'tmp_home=$(mktemp -d); HOME="$tmp_home" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run --status-level fail --final-status-level fail'

## Command

```console
sh -c 'tmp_home=$(mktemp -d); HOME="$tmp_home" CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run --status-level fail --final-status-level fail'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 450
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-eqq6/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.91s
────────────
 Nextest run ID 84e02348-09fa-45d5-8f34-738cc811ee5a with nextest profile: default
    Starting 708 tests across 9 binaries (4 tests skipped)
────────────
     Summary [  42.626s] 708 tests run: 708 passed (2 slow), 4 skipped
```
