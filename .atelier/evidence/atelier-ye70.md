---
created_at: "2026-10-17T19:07:26.879257688+00:00"
id: "atelier-ye70"
evidence_type: "test"
commit: "e6dcd73b6651da1f0658b89f7680e112dd3f3388"
result: "pass"
captured_at: "2026-10-17T19:06:59.972310900+00:00"
command: "cargo nextest run -j 4 --test cli_integration -E 'test(decisions::) and not test(issue_show_lists)'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S1"
  result: "pass"
- slug: "S3"
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
title: "Decision note, ADR and supersession CLI tests pass on master"
updated_at: "2026-10-17T19:07:26.880723561+00:00"
---

## Summary

Decision note, ADR and supersession CLI tests pass on master

## Command

```console
cargo nextest run -j 4 --test cli_integration -E 'test(decisions::) and not test(issue_show_lists)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 3190
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling syn v2.0.111
   Compiling getrandom v0.3.4
   Compiling num-traits v0.2.19
   Compiling once_cell v1.21.3
   Compiling zerocopy v0.8.31
   Compiling clap_builder v4.5.53
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier/crates/atelier-cli)
   Compiling wait-timeout v0.2.1
   Compiling bit-vec v0.8.0
   Compiling synstructure v0.13.2
   Compiling serde_derive v1.0.228
   Compiling zerofrom-derive v0.1.6
   Compiling yoke-derive v0.8.1
   Compiling zerovec-derive v0.11.2
   Compiling displaydoc v0.2.6
   Compiling rand_core v0.9.3
   Compiling tempfile v3.27.0
   Compiling zerofrom v0.1.8
   Compiling clap_derive v4.5.49
   Compiling quick-error v1.2.3
   Compiling fnv v1.0.7
   Compiling yoke v0.8.1
   Compiling rusty-fork v0.3.1
   Compiling rand v0.9.4
   Compiling zerovec v0.11.5
   Compiling zerotrie v0.2.3
   Compiling rand_xorshift v0.4.0
   Compiling bit-set v0.8.0
   Compiling tinystr v0.8.2
   Compiling potential_utf v0.1.4
   Compiling regex-syntax v0.8.8
   Compiling ppv-lite86 v0.2.21
   Compiling icu_collections v2.1.1
   Compiling serde v1.0.228
   Compiling icu_locale_core v2.1.1
   Compiling rand_chacha v0.9.0
   Compiling unarray v0.1.4
   Compiling arbitrary v1.4.2
   Compiling icu_provider v2.1.1
   Compiling clap v4.5.53
   Compiling icu_properties v2.1.2
   Compiling icu_normalizer v2.1.1
   Compiling proptest v1.9.0
   Compiling chrono v0.4.45
   Compiling serde_yaml v0.9.34+deprecated
   Compiling serde_spanned v0.6.9
   Compiling toml_datetime v0.6.11
   Compiling toml_edit v0.22.27
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier/crates/atelier-core)
   Compiling toml v0.8.23
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 18.99s
────────────
 Nextest run ID 1a3af9f3-942f-438f-8dac-dc1bcb637b76 with nextest profile: default
    Starting 7 tests across 1 binary (331 tests skipped)
        PASS [   0.174s] (1/7) atelier-cli::cli_integration decisions::decision_notes_refuse_missing_reasons_and_stray_flags
        PASS [   0.181s] (2/7) atelier-cli::cli_integration decisions::check_refuses_malformed_decision_notes
        PASS [   0.186s] (3/7) atelier-cli::cli_integration decisions::decision_notes_never_store_what_check_refuses
        PASS [   0.239s] (4/7) atelier-cli::cli_integration decisions::decision_notes_record_choice_reason_rejected_and_one_way
        PASS [   0.186s] (5/7) atelier-cli::cli_integration decisions::work_decisions_refuses_items_that_are_not_missions
        PASS [   0.341s] (6/7) atelier-cli::cli_integration decisions::long_decision_rows_continue_past_the_rows
        PASS [   1.909s] (7/7) atelier-cli::cli_integration decisions::decision_records_link_an_adr_and_show_supersession
────────────
     Summary [   2.095s] 7 tests run: 7 passed, 331 skipped
```

