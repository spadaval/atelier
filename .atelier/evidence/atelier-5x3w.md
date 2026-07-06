---
created_at: "2026-07-06T18:30:17.906847633+00:00"
id: "atelier-5x3w"
evidence_type: "test"
captured_at: "2026-07-06T18:30:10.424001783+00:00"
command: "cargo nextest run -p atelier-app -p atelier-cli -E 'test(persist_pull_request_writes_owner_epic_field_and_child_inherits) | test(pr_open_persists_link_and_records_action_after_preflight) | test(pr_link_fetches_remote_pull_and_persists_owner_field) | test(linked_pull_request_merge_status_reports_required_states) | test(test_issue_show_surfaces_evidence_status)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-hl1n"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-hl1n"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-app -p atelier-cli -E 'test(persist_pull_request_writes_owner_epic_field_and_child_inherits) | test(pr_open_persists_link_and_records_action_after_preflight) | test(pr_link_fetches_remote_pull_and_persists_owner_field) | test(linked_pull_request_merge_status_reports_required_states) | test(test_issue_show_surfaces_evidence_status)'"
updated_at: "2026-07-06T18:30:24.505041593+00:00"
---

## Summary

cargo nextest run -p atelier-app -p atelier-cli -E 'test(persist_pull_request_writes_owner_epic_field_and_child_inherits) | test(pr_open_persists_link_and_records_action_after_preflight) | test(pr_link_fetches_remote_pull_and_persists_owner_field) | test(linked_pull_request_merge_status_reports_required_states) | test(test_issue_show_surfaces_evidence_status)'

## Command

```console
cargo nextest run -p atelier-app -p atelier-cli -E 'test(persist_pull_request_writes_owner_epic_field_and_child_inherits) | test(pr_open_persists_link_and_records_action_after_preflight) | test(pr_link_fetches_remote_pull_and_persists_owner_field) | test(linked_pull_request_merge_status_reports_required_states) | test(test_issue_show_surfaces_evidence_status)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1077
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-hl1n/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/root/.codex/worktrees/mska-hl1n/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.73s
────────────
 Nextest run ID e63677e5-df05-4e57-b03a-e63381f0366c with nextest profile: default
    Starting 5 tests across 5 binaries (553 tests skipped)
        PASS [   0.166s] (1/5) atelier-app pr::tests::linked_pull_request_merge_status_reports_required_states
        PASS [   0.251s] (2/5) atelier-app pr::tests::persist_pull_request_writes_owner_epic_field_and_child_inherits
        PASS [   0.255s] (3/5) atelier-app pr::tests::pr_link_fetches_remote_pull_and_persists_owner_field
        PASS [   0.263s] (4/5) atelier-app pr::tests::pr_open_persists_link_and_records_action_after_preflight
        PASS [   2.452s] (5/5) atelier-cli::cli_integration issues::test_issue_show_surfaces_evidence_status
────────────
     Summary [   2.454s] 5 tests run: 5 passed, 553 skipped
```
