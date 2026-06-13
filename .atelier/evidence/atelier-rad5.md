---
created_at: "2026-06-12T22:28:03.391185887+00:00"
id: "atelier-rad5"
evidence_type: "validation"
captured_at: "2026-06-12T22:27:43.392298590+00:00"
command: "bash -lc '\nset -euo pipefail\nRUSTFLAGS=-Awarnings cargo nextest run --status-level pass --final-status-level pass --success-output never --failure-output final \\\n  mission_record_renders_and_parses_deterministically_without_data_blob \\\n  mission_render_normalizes_legacy_evidence_attachments \\\n  legacy_mission_data_record_loads_into_typed_sections_and_relationships \\\n  rebuild_round_trips_canonical_domain_records \\\n  test_first_class_detail_views_read_payloads_from_record_store \\\n  test_first_class_records_export_rebuild_and_validate \\\n  test_mission_relationship_filtering_keeps_supporting_records_out_of_work\n'"
exit_status: "0"
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6aor"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Focused mission record tests pass with concise output"
updated_at: "2026-06-12T22:28:04.660238994+00:00"
---

Focused mission record tests pass with concise output

Command: bash -lc '
set -euo pipefail
RUSTFLAGS=-Awarnings cargo nextest run --status-level pass --final-status-level pass --success-output never --failure-output final \
  mission_record_renders_and_parses_deterministically_without_data_blob \
  mission_render_normalizes_legacy_evidence_attachments \
  legacy_mission_data_record_loads_into_typed_sections_and_relationships \
  rebuild_round_trips_canonical_domain_records \
  test_first_class_detail_views_read_payloads_from_record_store \
  test_first_class_records_export_rebuild_and_validate \
  test_mission_relationship_filtering_keeps_supporting_records_out_of_work
'
Exit status: 0

Stdout summary:
(none)

Stderr summary (truncated):
   Compiling getrandom v0.3.4
   Compiling bitflags v2.10.0
   Compiling num-traits v0.2.19
   Compiling regex-syntax v0.8.8
   Compiling rustix v1.1.3
   Compiling zerocopy v0.8.31
   Compiling linux-raw-sys v0.11.0
   Compiling fastrand v2.3.0
   Compiling atelier-tracker v0.2.0 (/root/atelier)
   Compiling wait-timeout v0.2.1
   Compiling fnv v1.0.7
   Compiling bit-vec v0.8.0
   Compiling quick-error v1.2.3
   Compiling unarray v0.1.4
   Compiling arbitrary v1.4.2
   Compiling rusqlite v0.38.0
   Compiling bit-set v0.8.0
   Compiling rand_core v0.9.3
   Compiling rand_xorshift v0.4.0
   Compiling rand v0.9.4
   Compiling chrono v0.4.42
   Compiling regex-automata v0.4.14
   Compiling tempfile v3.24.0
   Compiling ppv-lite86 v0.2.21
   Compiling rusty-fork v0.3.1
   Compiling rand_chacha v0.9.0
   Compiling proptest v1.9.0
   Compiling matchers v0.2.0
   Compiling tracing-subscriber v0.3.23
    Finished `test` profile [unoptimized + debuginfo] target(s) in 18.16s
────────────
 Nextest run ID 33903ac3-0539-4b9b-9dcb-b5b1232255f3 with nextest profile: default
    Starting 10 tests across 4 binaries (766 tests skipped)
        PASS [   0.009s] ( 1/10) atelier-tracker::bin/atelier record_store::tests::mission_render_normalizes_legacy_evidence_attachments
        PASS [   0.009s] ( 2/10) atelier-tracker::bin/atelier record_store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships
        PASS [   0.011s] ( 3/10) atelier-tracker::bin/atelier record_store::tests::mission_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.011s] ( 4/10) atelier-tracker record_store::tests::mission_render_normalizes_legacy_evidence_attachments
        PASS [   0.012s] ( 5/10) atelier-tracker record_store::tests::mission_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.012s] ( 6/10) atelier-tracker record_store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships
        PASS [   0.291s] ( 7/10) atelier-tracker::bin/atelier commands::rebuild::tests::rebuild_round_trips_canonical_domain_records
        PASS [   0.509s] ( 8/10) atelier-tracker::cli_integration test_first_class_detail_views_read_payloads_from_record_store
        PASS [   0.945s] ( 9/10) atelier-tracker::cli_integration test_mission_relationship_filtering_keeps_supporting_records_out_of_work
        PASS [   1.437s] (10/10) atelier-tracker::cli_integration test_first_class_records_export_rebuild_and_validate
────────────
     Summary [   1.439s] 10 tests run: 10 passed, 766 skipped
        PASS [   0.009s] ( 1/10) atelier-tracker::bin/atelier record_store::tests::mission_render_normalizes_legacy_evidence_attachments
        PASS [   0.009s] ( 2/10) atelier-tracker::bin/atelier record_store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships
        PASS [   0.011s] ( 3/10) atelier-tracker::bin/atelier record_store::tests::mission_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.011s] ( 4/10) atelier-tracker record_store::tests::mission_render_normalizes_legacy_evidence_attachments
        PASS [   0.012s] ( 5/10) atelier-tracker record_store::tests::mission_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.012s] ( 6/10) atelier-tracker record_store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships
        PASS [   0.291s] ( 7/10) atelier-tracker::bin/atelier commands::rebuild::tests::rebuild_round_trips_canonical_domain_records
        PASS [   0.509s] ( 8/10) atelier-tracker::cli_integration test_first_class_detail_views_read_payloads_from_record_store
        PASS [   0.945s] ( 9/10) atelier-tracker::cli_integration test_mission_relationship_filtering_keeps_supporting_records_out_of_work
        PASS [   1.437s] (10/10) atelier-tracker::cli_integration test_first_class_records_export_rebuild_and_validate
        SKIP [         ] (─────) atelier-tracker activity::tests::allocation_adds_determin
