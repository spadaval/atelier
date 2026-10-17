---
created_at: "2026-10-17T20:02:49.700928247+00:00"
id: "atelier-yzm5"
evidence_type: "test"
commit: "458d4874e970122d9b08ba15d91c80411d66fac1"
result: "pass"
captured_at: "2026-10-17T20:02:40.368423215+00:00"
command: "cargo nextest run -j 4 --test cli_integration -E 'test(brief::)'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S5"
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
title: "Brief fixture tests of both forms, including plan approval shown in brief and debrief, pass on master"
updated_at: "2026-10-17T20:02:49.701444700+00:00"
---

## Summary

Brief fixture tests of both forms, including plan approval shown in brief and debrief, pass on master

## Command

```console
cargo nextest run -j 4 --test cli_integration -E 'test(brief::)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 2061
Truncated: no

```text
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier/crates/atelier-core)
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier/crates/atelier-cli)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.38s
────────────
 Nextest run ID 2cff15cd-f150-40d3-945d-f0ac499211a7 with nextest profile: default
    Starting 10 tests across 1 binary (338 tests skipped)
        PASS [   0.165s] ( 1/10) atelier-cli::cli_integration debrief::a_child_added_to_planned_scope_after_the_start_is_new_scope
        PASS [   0.209s] ( 2/10) atelier-cli::cli_integration brief::brief_says_no_plan_approval_in_both_forms_when_only_other_events_exist
        PASS [   0.209s] ( 3/10) atelier-cli::cli_integration brief::brief_html_holds_the_same_facts_as_the_markdown
        PASS [   0.248s] ( 4/10) atelier-cli::cli_integration brief::brief_markdown_shows_the_plan_and_its_approval
        PASS [   0.033s] ( 5/10) atelier-cli::cli_integration debrief::debrief_refuses_items_that_are_not_missions
        PASS [   0.147s] ( 6/10) atelier-cli::cli_integration debrief::a_link_stored_on_the_mission_dates_from_the_mission_record
        PASS [   0.269s] ( 7/10) atelier-cli::cli_integration debrief::proof_before_later_code_is_aged_up_to_the_close
        PASS [   0.520s] ( 8/10) atelier-cli::cli_integration debrief::debrief_markdown_shows_criteria_reports_decisions_deviations_and_follow_ups
        PASS [   0.540s] ( 9/10) atelier-cli::cli_integration debrief::debrief_html_is_a_self_contained_page_with_the_same_facts
        PASS [   1.337s] (10/10) atelier-cli::cli_integration debrief::work_merged_from_a_branch_is_not_reported_reopened
────────────
     Summary [   1.650s] 10 tests run: 10 passed, 338 skipped
```

