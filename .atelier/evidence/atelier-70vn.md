---
created_at: "2026-07-06T18:17:11.896763200+00:00"
id: "atelier-70vn"
evidence_type: "test"
captured_at: "2026-07-06T18:17:11.173532052+00:00"
command: "cargo nextest run -p atelier-records -E 'test(record_store_concrete_services_round_trip_and_discover_domain_files) | test(evidence_record_renders_and_parses_deterministically_without_data_blob) | test(review_room_record_renders_and_parses_yaml)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ax1g"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ax1g"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-records -E 'test(record_store_concrete_services_round_trip_and_discover_domain_files) | test(evidence_record_renders_and_parses_deterministically_without_data_blob) | test(review_room_record_renders_and_parses_yaml)'"
updated_at: "2026-07-06T18:17:26.736220604+00:00"
---

## Summary

cargo nextest run -p atelier-records -E 'test(record_store_concrete_services_round_trip_and_discover_domain_files) | test(evidence_record_renders_and_parses_deterministically_without_data_blob) | test(review_room_record_renders_and_parses_yaml)'

## Command

```console
cargo nextest run -p atelier-records -E 'test(record_store_concrete_services_round_trip_and_discover_domain_files) | test(evidence_record_renders_and_parses_deterministically_without_data_blob) | test(review_room_record_renders_and_parses_yaml)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 689
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
────────────
 Nextest run ID ce27f7e0-9738-4220-ac21-c3966cd075a0 with nextest profile: default
    Starting 3 tests across 1 binary (47 tests skipped)
        PASS [   0.014s] (1/3) atelier-records tests::evidence_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.016s] (2/3) atelier-records tests::record_store_concrete_services_round_trip_and_discover_domain_files
        PASS [   0.021s] (3/3) atelier-records tests::review_room_record_renders_and_parses_yaml
────────────
     Summary [   0.022s] 3 tests run: 3 passed, 47 skipped
```

