---
created_at: "2026-10-17T19:08:31.232871862+00:00"
id: "atelier-hox4"
evidence_type: "test"
commit: "e6dcd73b6651da1f0658b89f7680e112dd3f3388"
result: "pass"
captured_at: "2026-10-17T19:08:14.271648601+00:00"
command: "cargo nextest run -j 4 --test cli_integration --test session -E 'test(issue_show_lists_the_current) | test(status_shows_the_owner)'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S6"
  result: "pass"
criteria_mission: "atelier-r6bt"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-r6bt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "issue show and status decision view tests pass on master"
updated_at: "2026-10-17T19:08:31.233235740+00:00"
---

## Summary

issue show and status decision view tests pass on master

## Command

```console
cargo nextest run -j 4 --test cli_integration --test session -E 'test(issue_show_lists_the_current) | test(status_shows_the_owner)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 717
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.17s
────────────
 Nextest run ID fb05e607-b2a7-465c-b469-6318b4283ee5 with nextest profile: default
    Starting 2 tests across 2 binaries (425 tests skipped)
        PASS [   1.029s] (1/2) atelier-cli::session status_shows_the_owner_decisions_in_brief
        PASS [   1.191s] (2/2) atelier-cli::cli_integration decisions::issue_show_lists_the_current_decisions_bearing_on_an_item
────────────
     Summary [   1.191s] 2 tests run: 2 passed, 425 skipped
```

