---
created_at: "2026-10-17T19:47:29.624031374+00:00"
id: "atelier-9r9q"
evidence_type: "test"
commit: "8b6f17834e3667440ebde06d522865d5968bcb3c"
result: "pass"
captured_at: "2026-10-17T19:47:20.251868405+00:00"
command: "sh -c 'cargo nextest run -j 4 --test cli_integration -E '\"'\"'test(debrief::)'\"'\"' && atelier debrief atelier-kgcr >/dev/null && atelier debrief atelier-kgcr --html -q'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S4"
  result: "pass"
criteria_mission: "atelier-r6bt"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-r6bt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Debrief fixture tests of both forms, the facts-match test and the history fixtures pass on master; debrief of the done mission atelier-kgcr runs"
updated_at: "2026-10-17T19:47:29.624311496+00:00"
---

## Summary

Debrief fixture tests of both forms, the facts-match test and the history fixtures pass on master; debrief of the done mission atelier-kgcr runs

## Command

```console
sh -c 'cargo nextest run -j 4 --test cli_integration -E '"'"'test(debrief::)'"'"' && atelier debrief atelier-kgcr >/dev/null && atelier debrief atelier-kgcr --html -q'
```

Exit status: 0

## Stdout

Bytes: 83
Truncated: no

```text
/home/supadava/projects/atelier/.atelier/runtime/reviews/atelier-kgcr-debrief.html
```

## Stderr

Bytes: 1396
Truncated: no

```text
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.31s
────────────
 Nextest run ID 58c4a63c-3cda-439a-bcd6-91394b2b2dad with nextest profile: default
    Starting 7 tests across 1 binary (338 tests skipped)
        PASS [   0.100s] (1/7) atelier-cli::cli_integration debrief::a_link_stored_on_the_mission_dates_from_the_mission_record
        PASS [   0.110s] (2/7) atelier-cli::cli_integration debrief::a_child_added_to_planned_scope_after_the_start_is_new_scope
        PASS [   0.024s] (3/7) atelier-cli::cli_integration debrief::debrief_refuses_items_that_are_not_missions
        PASS [   0.181s] (4/7) atelier-cli::cli_integration debrief::proof_before_later_code_is_aged_up_to_the_close
        PASS [   0.375s] (5/7) atelier-cli::cli_integration debrief::debrief_markdown_shows_criteria_reports_decisions_deviations_and_follow_ups
        PASS [   0.379s] (6/7) atelier-cli::cli_integration debrief::debrief_html_is_a_self_contained_page_with_the_same_facts
        PASS [   1.256s] (7/7) atelier-cli::cli_integration debrief::work_merged_from_a_branch_is_not_reported_reopened
────────────
     Summary [   1.380s] 7 tests run: 7 passed, 338 skipped
```

