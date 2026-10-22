---
created_at: "2026-10-22T23:18:35.521430441+00:00"
id: "atelier-xize"
evidence_type: "test"
commit: "cb30c86b6f001bebb33734a96903515f74bc2f78"
result: "pass"
captured_at: "2026-10-22T23:18:31.531417756+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks a_base_exemption"
exit_status: "0"
agent_identity: "claude-implementer-mbrf"
target:
  kind: "issue"
  id: "atelier-mbrf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mbrf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Base exemption covers exactly its commit for land (unexempted and later commits refused, done claim still refused), round-trips as a base: record, and shows on the debrief timeline"
updated_at: "2026-10-22T23:18:35.521857877+00:00"
---

## Summary

Base exemption covers exactly its commit for land (unexempted and later commits refused, done claim still refused), round-trips as a base: record, and shows on the debrief timeline

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks a_base_exemption
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 806
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-j8kg/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.83s
────────────
 Nextest run ID 60216f82-84e9-445f-aafd-13e2a0c13c91 with nextest profile: default
    Starting 3 tests across 10 binaries (874 tests skipped)
        PASS [   0.009s] (1/3) atelier-records review::tests::a_base_exemption_names_a_base_instead_of_a_target_and_round_trips
        PASS [   0.221s] (2/3) atelier-cli::cli_integration debrief::a_base_exemption_made_while_the_mission_ran_is_on_its_timeline
        PASS [   0.700s] (3/3) atelier-cli::session a_base_exemption_covers_exactly_its_commit
────────────
     Summary [   0.701s] 3 tests run: 3 passed, 874 skipped
```

