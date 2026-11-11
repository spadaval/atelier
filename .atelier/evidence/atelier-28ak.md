---
created_at: "2026-11-11T01:22:01.554520130+00:00"
id: "atelier-28ak"
evidence_type: "test"
commit: "f7ec8a3487c3b2e788056f0b5a91e4365a9356ed"
result: "fail"
captured_at: "2026-11-11T01:21:48.980704535+00:00"
command: "bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings && cargo nextest run -j 4 --locked && cd .. && git diff --check master... && test -z \"$(git diff --name-only master... -- crates/)\"'"
exit_status: "2"
target:
  kind: "issue"
  id: "atelier-t2jo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t2jo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings && cargo nextest run -j 4 --locked && cd .. && git diff --check master... && test -z \"$(git diff --name-only master... -- crates/)\"'"
updated_at: "2026-11-11T01:22:01.706390615+00:00"
---

## Summary

bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings && cargo nextest run -j 4 --locked && cd .. && git diff --check master... && test -z "$(git diff --name-only master... -- crates/)"'

## Command

```console
bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings && cargo nextest run -j 4 --locked && cd .. && git diff --check master... && test -z "$(git diff --name-only master... -- crates/)"'
```

Exit status: 2

## Stdout

Bytes: 60
Truncated: no
SHA-256: e1b0875386beedc61e84771697e1a430afa608d9acbc2a857f772ec03b9d70f0

```text
.atelier/issues/atelier-t2jo.md:111: new blank line at EOF.
```

## Stderr

Bytes: 26719
Truncated: yes
SHA-256: 71b16dd4675a45a44f5a3f75f9bcbd933c1b0ee5b09511dcd4aea2868ed9bbde
Test summary: Summary [  11.784s] 223 tests run: 223 passed, 0 skipped

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
────────────
 Nextest run ID 6c4b55ff-a1d4-45fb-805c-811af94a97d5 with nextest profile: default
    Starting 223 tests across 17 binaries
        PASS [   0.005s] (  1/223) atelier-app::acts a_finding_is_settled_only_when_its_item_is_done_and_it_is_deferred_and_unsettled
        PASS [   0.006s] (  2/223) atelier-app identity::tests::takes_the_first_variable_set_in_order_else_human
        PASS [   0.006s] (  3/223) atelier-app::acts a_kept_finding_whose_rebuttal_was_kept_has_had_its_one_try
        PASS [   0.006s] (  4/223) atelier-app::acts a_plan_verdict_counts_its_own_file_and_earlier_rounds_not_a_blind_one
        PASS [   0.004s] (  5/223) atelier-app::acts closing_a_mission_needs_each_item_done_or_dropped_and_its_result_written
        PASS [   0.005s] (  6/223) atelier-app::acts a_round_carries_what_earlier_rounds_left_open_with_each_
… 22623 bytes omitted …
_and_its_directorys_name
        PASS [   0.007s] (197/223) atelier-records::records a_mission_needs_an_outcome_section
        PASS [   0.005s] (198/223) atelier-records::records a_mission_reads_its_fields_and_keeps_its_body
        PASS [   0.005s] (199/223) atelier-records::records a_mission_with_an_unknown_status_is_refused_naming_file_and_field
        PASS [   0.005s] (200/223) atelier-records::records a_missions_other_files_are_not_examined
        PASS [   0.005s] (201/223) atelier-records::records a_review_with_no_model_reads_and_writes_back_as_null
        PASS [   0.007s] (202/223) atelier-records::records a_missions_plan_reviews_are_read_with_it_and_checked_as_an_items_are
        PASS [   0.008s] (203/223) atelier-records::records an_approve_verdict_is_refused_while_its_file_holds_an_open_fix_now_finding
        PASS [   0.005s] (204/223) atelier-records::records an_item_gives_its_sections_and_a_review_each_finding_as_written
        PASS [   0.006s] (205/223) atelier-records::records an_item_needs_goal_and_done_when
        PASS [   0.004s] (206/223) atelier-records::records an_unknown_front_matter_key_is_refused_naming_it
        PASS [   0.007s] (207/223) atelier-records::records an_items_mission_must_name_a_mission
        PASS [   0.004s] (208/223) atelier-records::records an_unknown_status_is_refused_naming_file_and_field
        PASS [   0.005s] (209/223) atelier-records::records config_reads_its_fields
        PASS [   0.005s] (210/223) atelier-records::records config_refuses_what_it_does_not_know
        PASS [   0.010s] (211/223) atelier-records::records caused_by_holds_an_item_id_or_a_commit_hash_checked_for_form_only
        PASS [   0.005s] (212/223) atelier-records::records creating_reviews_of_a_round_gives_each_its_own_name
        PASS [   0.006s] (213/223) atelier-records::records each_record_kind_reads_and_writes_back_the_same_file
        PASS [   0.348s] (214/223) atelier-cli::views list_shows_open_work_by_mission_and_label_and_each_flag_narrows_it
        PASS [   0.006s] (215/223) atelier-records::records field_lines_in_a_fenced_example_are_text
        PASS [   0.005s] (216/223) atelier-records::records findings_are_named_by_review_stem_and_number
        PASS [   0.005s] (217/223) atelier-records::records implementers_and_reviewers_are_lists_whose_last_entry_holds_the_role
        PASS [   0.006s] (218/223) atelier-records::records inline_code_on_a_line_of_its_own_opens_no_fence
        PASS [   0.008s] (219/223) atelier-records::records settling_lines_in_another_form_are_refused
        PASS [   0.006s] (220/223) atelier-records::records the_example_tree_is_valid
        PASS [   0.005s] (221/223) atelier-records::records the_store_writes_a_mission_where_it_reads_it
        PASS [   0.015s] (222/223) atelier-records::records records_whose_names_or_lines_are_ambiguous_are_refused
        PASS [   0.006s] (223/223) atelier-records::records the_store_writes_records_where_it_reads_them
────────────
     Summary [  11.784s] 223 tests run: 223 passed, 0 skipped
```
