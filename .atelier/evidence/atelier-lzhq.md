---
created_at: "2026-10-22T20:56:02.083526087+00:00"
id: "atelier-lzhq"
evidence_type: "test"
commit: "69bd49293954e3171c853f8e014ca04b67e7288b"
result: "pass"
captured_at: "2026-10-22T20:56:00.107386768+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(land_names_unpushed) | test(land_push) | test(land_ends)'"
exit_status: "0"
agent_identity: "claude-implementer-mvzi"
target:
  kind: "issue"
  id: "atelier-mvzi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mvzi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land names unpushed base commits the owner did not make, before the push"
updated_at: "2026-10-22T20:56:02.083802362+00:00"
---

## Summary

land names unpushed base commits the owner did not make, before the push

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(land_names_unpushed) | test(land_push) | test(land_ends)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 791
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-yim7/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.40s
────────────
 Nextest run ID 0947e04b-47a3-4545-99e6-5171e6657cc7 with nextest profile: default
    Starting 3 tests across 6 binaries (628 tests skipped)
        PASS [   0.253s] (1/3) atelier-cli::session land_push_reports_a_failed_push_and_a_missing_upstream
        PASS [   0.277s] (2/3) atelier-cli::session land_names_unpushed_base_commits_the_owner_did_not_make_before_pushing
        PASS [   0.289s] (3/3) atelier-cli::session land_ends_with_a_verdict_line_and_push_follows_only_a_pass
────────────
     Summary [   0.290s] 3 tests run: 3 passed, 628 skipped
```

