---
created_at: "2026-10-23T14:41:26.598005191+00:00"
id: "atelier-3a96"
evidence_type: "test"
commit: "fa47f83a4dbb588e03d70cbfeed54763ddd47cbc"
result: "pass"
captured_at: "2026-10-23T14:41:23.746475984+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_pass_lists_what_it_checked_and_the_next_step)'"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-bvl3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-bvl3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "passing land: records committed, owner checks with approval, done claims held, Next: atelier merge for a done owner; active owner names first five uncommitted paths and atelier status; quiet prints passed"
updated_at: "2026-10-23T14:41:26.598418351+00:00"
---

## Summary

passing land: records committed, owner checks with approval, done claims held, Next: atelier merge for a done owner; active owner names first five uncommitted paths and atelier status; quiet prints passed

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_pass_lists_what_it_checked_and_the_next_step)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 551
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.07s
────────────
 Nextest run ID 7ab4a99b-2a2b-4362-a8e1-114d5c68abb1 with nextest profile: default
    Starting 1 test across 10 binaries (897 tests skipped)
        PASS [   0.479s] (1/1) atelier-cli::session land_pass_lists_what_it_checked_and_the_next_step
────────────
     Summary [   0.480s] 1 test run: 1 passed, 897 skipped
```

