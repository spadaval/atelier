---
created_at: "2026-07-07T05:45:28.065662156+00:00"
id: "atelier-9mto"
evidence_type: "test"
captured_at: "2026-07-07T05:45:26.102543501+00:00"
command: "cargo test -p atelier-cli --lib mission_overview_render_tests --no-fail-fast"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-dy3u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-dy3u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission Overview renderer snapshot and color/plain semantic parity"
updated_at: "2026-07-07T05:45:32.375929798+00:00"
---

## Summary

Mission Overview renderer snapshot and color/plain semantic parity

## Command

```console
cargo test -p atelier-cli --lib mission_overview_render_tests --no-fail-fast
```

Exit status: 0

## Stdout

Bytes: 488
Truncated: no

```text

running 3 tests
test commands::work::mission_overview_render_tests::empty_overview_keeps_a_record_browsing_drill_down ... ok
test commands::work::mission_overview_render_tests::mission_overview_panel_renders_hierarchy_rollups_omissions_and_drill_downs ... ok
test commands::work::mission_overview_render_tests::mission_overview_color_is_semantic_only_and_no_color_preserves_meaning ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 140 filtered out; finished in 0.00s
```

## Stderr

Bytes: 359
Truncated: no

```text
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-overview/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.88s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
```

