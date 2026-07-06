---
created_at: "2026-07-06T17:59:26.082646670+00:00"
id: "atelier-lzdn"
evidence_type: "test"
captured_at: "2026-07-06T17:59:20.965093346+00:00"
command: "cargo nextest run -E 'test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_default_output) or test(test_evidence_quiet_output_is_a_stable_id_composition_path)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-9evg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9evg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Evidence browse budget: default list cap, transcript elision, full quiet ID composition, and focused inspection pass"
updated_at: "2026-07-06T17:59:30.068168485+00:00"
---

## Summary

Evidence browse budget: default list cap, transcript elision, full quiet ID composition, and focused inspection pass

## Command

```console
cargo nextest run -E 'test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_default_output) or test(test_evidence_quiet_output_is_a_stable_id_composition_path)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 819
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-yysm/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.10s
────────────
 Nextest run ID c5bbf152-23fa-45a8-96e3-c62e8ad1478e with nextest profile: default
    Starting 3 tests across 9 binaries (718 tests skipped)
        PASS [   0.286s] (1/3) atelier-cli::cli_integration records_evidence::test_evidence_list_elides_command_transcripts
        PASS [   0.618s] (2/3) atelier-cli::cli_integration records_evidence::test_evidence_quiet_output_is_a_stable_id_composition_path
        PASS [   2.708s] (3/3) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_default_output
────────────
     Summary [   2.711s] 3 tests run: 3 passed, 718 skipped
```
