---
created_at: "2026-10-22T21:24:35.419092483+00:00"
id: "atelier-z10s"
evidence_type: "test"
commit: "b41a458a73a7c604f13abb88b371bb0715341e79"
result: "pass"
captured_at: "2026-10-22T21:24:33.257215034+00:00"
command: "cargo nextest run -j 4 -E 'test(discovered_from)'"
exit_status: "0"
agent_identity: "claude-implementer-tq1j"
target:
  kind: "issue"
  id: "atelier-tq1j"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tq1j"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "discovered_from CLI tests: create flag, show, missing source, deferral agreement, debrief by source"
updated_at: "2026-10-22T21:24:35.419996819+00:00"
---

## Summary

discovered_from CLI tests: create flag, show, missing source, deferral agreement, debrief by source

## Command

```console
cargo nextest run -j 4 -E 'test(discovered_from)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1241
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-zuow/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.45s
────────────
 Nextest run ID baa03245-77a4-4afd-b051-4828e6c88a09 with nextest profile: default
    Starting 6 tests across 10 binaries (816 tests skipped)
        PASS [   0.063s] (1/6) atelier-cli::cli_integration discovered_from::a_missing_source_is_refused_at_create_and_by_check
        PASS [   0.079s] (2/6) atelier-cli::cli_integration discovered_from::a_deferral_to_a_follow_up_found_in_the_same_item_passes_check
        PASS [   0.088s] (3/6) atelier-cli::cli_integration discovered_from::a_deferral_must_name_the_source_its_follow_up_declares
        PASS [   0.114s] (4/6) atelier-cli::cli_integration discovered_from::a_finding_or_a_receipt_can_be_the_source
        PASS [   0.090s] (5/6) atelier-cli::cli_integration discovered_from::create_stores_the_link_and_show_prints_the_source
        PASS [   0.164s] (6/6) atelier-cli::cli_integration discovered_from::the_debrief_lists_discovered_work_by_source_without_an_advances_link
────────────
     Summary [   0.245s] 6 tests run: 6 passed, 816 skipped
```

