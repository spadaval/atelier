---
created_at: "2026-06-21T18:59:14.042238515+00:00"
id: "atelier-c5vr"
evidence_type: "test"
captured_at: "2026-06-21T18:59:11.121843862+00:00"
command: "cargo nextest run -p atelier-cli test_issue_link_replaces_objective_relationship_mutations test_issue_status_includes_linked_issue_hierarchy"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-fyc9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fyc9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Focused relationship tests for issue link and objective status"
updated_at: "2026-06-21T18:59:18.761497914+00:00"
---

## Summary

Focused relationship tests for issue link and objective status

## Command

```console
cargo nextest run -p atelier-cli test_issue_link_replaces_objective_relationship_mutations test_issue_status_includes_linked_issue_hierarchy
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 681
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-vays/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.90s
────────────
 Nextest run ID 0f869061-3acb-43a4-99a8-44633a85c6a1 with nextest profile: default
    Starting 2 tests across 4 binaries (440 tests skipped)
        PASS [   0.545s] (1/2) atelier-cli::cli_integration setup_guidance::test_issue_status_includes_linked_issue_hierarchy
        PASS [   0.726s] (2/2) atelier-cli::cli_integration setup_guidance::test_issue_link_replaces_objective_relationship_mutations
────────────
     Summary [   0.727s] 2 tests run: 2 passed, 440 skipped
```

