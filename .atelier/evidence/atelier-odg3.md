---
created_at: "2026-06-23T23:32:32.137253279+00:00"
id: "atelier-odg3"
evidence_type: "validation"
captured_at: "2026-06-23T23:32:23.344164220+00:00"
command: "sh -c 'cargo fmt -- --check && cargo check -p atelier-cli && cargo nextest run -p atelier-cli -E '\"'\"'test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_default_output) or test(test_history_repo_wide_supports_filters_bounded_output_and_drill_downs)'\"'\"' && target/debug/atelier lint atelier-c0qc && target/debug/atelier export --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7fof"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7fof"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Evidence and history browse output validation"
updated_at: "2026-06-23T23:32:37.327840326+00:00"
---

## Summary

Evidence and history browse output validation

## Command

```console
sh -c 'cargo fmt -- --check && cargo check -p atelier-cli && cargo nextest run -p atelier-cli -E '"'"'test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_default_output) or test(test_history_repo_wide_supports_filters_bounded_output_and_drill_downs)'"'"' && target/debug/atelier lint atelier-c0qc && target/debug/atelier export --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 71
Truncated: no

```text
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1083
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.97s
────────────
 Nextest run ID 05f1b165-125a-49e0-a965-3053e6e241f4 with nextest profile: default
    Starting 4 tests across 4 binaries (470 tests skipped)
        PASS [   0.400s] (1/4) atelier-cli::cli_integration records_evidence::test_evidence_list_elides_command_transcripts
        PASS [   0.547s] (2/4) atelier-cli::cli_integration issues::test_history_repo_wide_supports_filters_bounded_output_and_drill_downs
        PASS [   2.213s] (3/4) atelier-cli::cli_integration records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets
        PASS [   2.667s] (4/4) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_default_output
────────────
     Summary [   2.668s] 4 tests run: 4 passed, 470 skipped
```

