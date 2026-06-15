---
created_at: "2026-06-15T17:16:11.869013983+00:00"
id: "atelier-q2l9"
evidence_type: "validation"
captured_at: "2026-06-15T17:15:49.398939491+00:00"
command: "sh -c 'cargo test -p atelier-sqlite && target/debug/atelier issue list --ready >/tmp/atelier-xmvz-ready.txt && target/debug/atelier search sqlite >/tmp/atelier-xmvz-search.txt && target/debug/atelier graph tree >/tmp/atelier-xmvz-graph.txt && target/debug/atelier mission status atelier-v5nb >/tmp/atelier-xmvz-mission.txt'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-xmvz"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 470
    summary: "\nrunning 4 tests\ntest tests::removed_tables_are_not_part_of_target_schema ... ok\ntest tests::schema_tables_have_explicit_ownership ... ok\ntest tests::projection_index_reports_source_freshness ... ok\ntest tests::projection_index_stores_and_queries_issues ... ok\n\ntest result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n"
    truncated: false
  stderr:
    bytes: 187
    summary: "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s\n     Running unittests src/lib.rs (target/debug/deps/atelier_sqlite-88b46a0edae2c1f3)\n   Doc-tests atelier_sqlite\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xmvz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-sqlite owns projection freshness and representative query APIs; focused crate tests cover rebuild-style source freshness, ready/blocked/search/blocker queries, and CLI ready/search/graph/mission status commands run against the rebuilt projection."
updated_at: "2026-06-15T17:16:15.334818755+00:00"
---

atelier-sqlite owns projection freshness and representative query APIs; focused crate tests cover rebuild-style source freshness, ready/blocked/search/blocker queries, and CLI ready/search/graph/mission status commands run against the rebuilt projection.

Command: sh -c 'cargo test -p atelier-sqlite && target/debug/atelier issue list --ready >/tmp/atelier-xmvz-ready.txt && target/debug/atelier search sqlite >/tmp/atelier-xmvz-search.txt && target/debug/atelier graph tree >/tmp/atelier-xmvz-graph.txt && target/debug/atelier mission status atelier-v5nb >/tmp/atelier-xmvz-mission.txt'
Exit status: 0

Stdout summary:

running 4 tests
test tests::removed_tables_are_not_part_of_target_schema ... ok
test tests::schema_tables_have_explicit_ownership ... ok
test tests::projection_index_reports_source_freshness ... ok
test tests::projection_index_stores_and_queries_issues ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Stderr summary:
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_sqlite-88b46a0edae2c1f3)
   Doc-tests atelier_sqlite

