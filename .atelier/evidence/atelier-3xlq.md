---
created_at: "2026-10-23T14:32:58.854054346+00:00"
id: "atelier-3xlq"
evidence_type: "test"
commit: "e7e76fde3096864c0735f705a8c742f5de060599"
result: "pass"
captured_at: "2026-10-23T14:32:56.175904920+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks review_show_is_a_brief_view_with_linked_events a_base_exemption_covers_exactly_its_commit short_ids_resolve_ignoring_case"
exit_status: "0"
agent_identity: "claude-implementer-phfo"
target:
  kind: "issue"
  id: "atelier-3u2u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3u2u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "review show: brief header, a finding with two dispositions, a disposition with its finding and supersedes, an approval marked current, short IDs, base exemption"
updated_at: "2026-10-23T14:32:58.854397596+00:00"
---

## Summary

review show: brief header, a finding with two dispositions, a disposition with its finding and supersedes, an approval marked current, short IDs, base exemption

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks review_show_is_a_brief_view_with_linked_events a_base_exemption_covers_exactly_its_commit short_ids_resolve_ignoring_case
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 804
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-phfo/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.90s
────────────
 Nextest run ID 8d722dcf-8996-48d7-b1f3-a989181e9f06 with nextest profile: default
    Starting 3 tests across 10 binaries (897 tests skipped)
        PASS [   0.211s] (1/3) atelier-cli::cli_integration record_refs::short_ids_resolve_ignoring_case_for_issues_evidence_and_reviews
        PASS [   0.250s] (2/3) atelier-cli::cli_integration review_views::review_show_is_a_brief_view_with_linked_events
        PASS [   0.492s] (3/3) atelier-cli::session a_base_exemption_covers_exactly_its_commit
────────────
     Summary [   0.494s] 3 tests run: 3 passed, 897 skipped
```

