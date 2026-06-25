---
created_at: "2026-06-25T00:59:09.732107756+00:00"
id: "atelier-blx4"
evidence_type: "test"
captured_at: "2026-06-25T00:59:05.219539274+00:00"
command: "cargo nextest run -p atelier-cli test_issue_status_does_not_report_done_children_as_open_work"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-qcbx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qcbx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli test_issue_status_does_not_report_done_children_as_open_work"
updated_at: "2026-06-25T00:59:12.931246522+00:00"
---

## Summary

cargo nextest run -p atelier-cli test_issue_status_does_not_report_done_children_as_open_work

## Command

```console
cargo nextest run -p atelier-cli test_issue_status_does_not_report_done_children_as_open_work
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 691
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.56s
────────────
 Nextest run ID 4e87ea85-9d77-41c3-808c-a6ac04fa5a5b with nextest profile: default
    Starting 1 test across 4 binaries (479 tests skipped)
        PASS [   1.639s] (1/1) atelier-cli::cli_integration setup_guidance::test_issue_status_does_not_report_done_children_as_open_work
────────────
     Summary [   1.640s] 1 test run: 1 passed, 479 skipped
```

