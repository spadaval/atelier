---
created_at: "2026-10-22T20:53:44.874402082+00:00"
id: "atelier-3ne9"
evidence_type: "test"
commit: "d6c662c60abb382dad4e039c3d323584194aebcf"
result: "pass"
captured_at: "2026-10-22T20:53:42.608186971+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(land_ends) | test(land_push) | test(setup_is_atomic)'"
exit_status: "0"
agent_identity: "claude-implementer-ecah"
target:
  kind: "issue"
  id: "atelier-ecah"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ecah"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land --push: verdict line, push only after pass, failed push, no upstream, hook pipefail"
updated_at: "2026-10-22T20:53:44.874805373+00:00"
---

## Summary

land --push: verdict line, push only after pass, failed push, no upstream, hook pipefail

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(land_ends) | test(land_push) | test(setup_is_atomic)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 828
Truncated: no

```text
    Blocking waiting for file lock on package cache
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-yim7/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.67s
────────────
 Nextest run ID c6fd3220-5c45-4688-9c21-31457d87a65a with nextest profile: default
    Starting 3 tests across 6 binaries (627 tests skipped)
        PASS [   0.123s] (1/3) atelier-cli::session setup_is_atomic_idempotent_and_preserves_other_handlers
        PASS [   0.231s] (2/3) atelier-cli::session land_push_reports_a_failed_push_and_a_missing_upstream
        PASS [   0.258s] (3/3) atelier-cli::session land_ends_with_a_verdict_line_and_push_follows_only_a_pass
────────────
     Summary [   0.260s] 3 tests run: 3 passed, 627 skipped
```

