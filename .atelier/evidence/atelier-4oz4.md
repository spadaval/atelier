---
created_at: "2026-07-06T17:59:07.030506041+00:00"
id: "atelier-4oz4"
evidence_type: "test"
captured_at: "2026-07-06T17:59:03.847504487+00:00"
command: "cargo nextest run -E 'test(test_issue_to_issue_validates_link_is_rejected) or test(test_evidence_relation_role_errors_are_corrective)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3g1y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3g1y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Evidence attach ownership: typed reuse survives; generic issue validates links reject; focused tests pass"
updated_at: "2026-07-06T17:59:11.795784467+00:00"
---

## Summary

Evidence attach ownership: typed reuse survives; generic issue validates links reject; focused tests pass

## Command

```console
cargo nextest run -E 'test(test_issue_to_issue_validates_link_is_rejected) or test(test_evidence_relation_role_errors_are_corrective)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 682
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-yysm/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.05s
────────────
 Nextest run ID d3dc1837-38e6-4b00-8f9e-bcce554d7a62 with nextest profile: default
    Starting 2 tests across 9 binaries (719 tests skipped)
        PASS [   0.411s] (1/2) atelier-cli::cli_integration issues::test_issue_to_issue_validates_link_is_rejected
        PASS [   0.816s] (2/2) atelier-cli::cli_integration records_evidence::test_evidence_relation_role_errors_are_corrective
────────────
     Summary [   0.817s] 2 tests run: 2 passed, 719 skipped
```

