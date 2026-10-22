---
created_at: "2026-10-22T23:21:14.984237296+00:00"
id: "atelier-3lwp"
evidence_type: "test"
commit: "81c707bb9dbff085e061a7410a488cf8eaf39ae4"
result: "pass"
captured_at: "2026-10-22T23:21:12.137931608+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks one_approval_on_the_parent"
exit_status: "0"
agent_identity: "claude-implementer-u6f3"
target:
  kind: "issue"
  id: "atelier-u6f3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u6f3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "One approval on the parent covers three children closed on its branch through land, merge and land on the base"
updated_at: "2026-10-22T23:21:14.984593630+00:00"
---

## Summary

One approval on the parent covers three children closed on its branch through land, merge and land on the base

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks one_approval_on_the_parent
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 567
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-j8kg/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.86s
────────────
 Nextest run ID ea21ded6-a7c3-4591-a3a4-00951afb6f5a with nextest profile: default
    Starting 1 test across 10 binaries (877 tests skipped)
        PASS [   0.724s] (1/1) atelier-cli::session one_approval_on_the_parent_covers_follow_ups_closed_on_its_branch
────────────
     Summary [   0.725s] 1 test run: 1 passed, 877 skipped
```

