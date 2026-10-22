---
created_at: "2026-10-22T20:50:34.091850292+00:00"
id: "atelier-l8hc"
evidence_type: "test"
commit: "faed7efe24d2fd026f108b34b6654b2b4a67e848"
result: "pass"
captured_at: "2026-10-22T20:50:31.859489089+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session -E 'test(conflict_resolving_merge) | test(without_findable_history)'"
exit_status: "0"
agent_identity: "claude-implementer-as7i"
target:
  kind: "issue"
  id: "atelier-as7i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-as7i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A review record and issue first added by a conflict-resolving merge are found and the no_action disposition accepted by land and doctor; a disposition with untraceable history fails findings.disposed naming it instead of aborting"
updated_at: "2026-10-22T20:50:34.092386822+00:00"
---

## Summary

A review record and issue first added by a conflict-resolving merge are found and the no_action disposition accepted by land and doctor; a disposition with untraceable history fails findings.disposed naming it instead of aborting

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session -E 'test(conflict_resolving_merge) | test(without_findable_history)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 679
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-1ztv/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.76s
────────────
 Nextest run ID df654409-7ba7-4474-9e2f-4df9e5a6220d with nextest profile: default
    Starting 2 tests across 1 binary (89 tests skipped)
        PASS [   0.157s] (1/2) atelier-cli::session a_disposition_without_findable_history_fails_its_gate_by_name
        PASS [   0.239s] (2/2) atelier-cli::session review_records_first_added_by_a_conflict_resolving_merge_are_found
────────────
     Summary [   0.239s] 2 tests run: 2 passed, 89 skipped
```

