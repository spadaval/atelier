---
created_at: "2026-11-10T00:56:06.471222574+00:00"
id: "atelier-mal9"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-mal9
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Read, write and validate item, review and config records in the new layout"
updated_at: "2026-11-10T15:36:19.838251337+00:00"
---

## Goal

Build the record store the new commands stand on: `items/<id>/item.md`, `reviews/<round>-<suffix>.md` and `config.toml` at schema version 3, as the architecture specification's Records section defines them. This is the first code of the full rewrite, so it also creates the new workspace, `next/`, with the crates `atelier-core` and `atelier-records` at `next/crates/<name>/`.

## Done when

- [x] Unit tests parse and write each record kind and get the same file back, with the Markdown body preserved byte for byte.
- [x] Validation rejects each of these with a message naming the file and the field: an unknown status; an item with no Goal or no Done when section; a finding with no Recommendation line; a review whose verdict is approve while it holds an open fix-now finding; a `blocked_by` that names no item.
- [x] An item's `implementers` and a review's `reviewers` are lists; unit tests cover appending a thread and reading the last entry as the holder.
- [x] A finding's name is its review's file stem and its number, as in `2-m2bn.1`; unit tests parse `Was:`, Responses lines and a Decisions entry's `Finding:` in that form, and reject a name no review has.
- [x] An item record carries an optional `caused_by`, holding an item ID or a commit hash. It is checked for form only, since the item that caused a bug may have left the tree.
- [x] Unit tests cover when a deferred finding is settled: it is unsettled until its text has a `Taken: <item> by <thread>` or a `Dropped by <thread>: <reason>` line, and both lines parse and write back unchanged.
- [x] Unit tests cover when a fix-now finding is open: it stays open until a later round marks it fixed or withdrawn, or a Decisions entry naming it has an outcome.
- [x] Creating a review for round 2 writes `reviews/2-<suffix>.md` with four random characters, and two reviews created for the same round have different names.
- [x] `config.toml` parsing covers the base branch, the merge strategy, merge checks, and a forge with and without `ci_workflow`; an unknown key is an error that names it.
- [x] `next/` is a Cargo workspace of its own with its own lock file, and `next/target/` is ignored by Git. The CI job that builds and tests `next/` deletes the root `crates/` directory before it builds, so the new workspace cannot build from the old one by any route. The Report shows that job's steps, run as written in a scratch clone, failing on a throwaway commit that adds a path dependency on an old crate.
- [x] `.github/workflows/ci.yml` has jobs that build and test `next/`. `scripts/ci-jobs.sh` says which workspace a change needs, the new one when a path under `next/` changes and the old one when a path under `crates/` does, and `scripts/ci-jobs.test.sh` covers both. No file under `crates/` is changed.
- [x] `scripts/count_non_test_rust.py` takes a directory and counts only under it, so the two workspaces can be reported apart.
- [x] The Records section of `docs/strategies/redesign-architecture.md` matches what was built; the Report lists each difference corrected there.

## Scope

Item, review and config records. Mission records, and checking that an item's `mission` names one, belong to step B. No CLI. Front-matter splitting is copied from `split_document` in `crates/atelier-records/src/lib.rs` with one change: it keeps the body as the raw slice, where today it strips one newline at each end, so a body with and without those newlines stays distinguishable; a unit test covers both. ID generation is copied from `crates/atelier-records/src/record_id.rs`. Both are trimmed to what the new records use and reviewed as new code. Nothing else is copied; the rest is written new.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records and Crates. The two pieces to copy are in `crates/atelier-records/src/lib.rs` and `crates/atelier-records/src/record_id.rs`. The rule for copying is in the architecture specification's section "The rule for this work". This is the first item of step A in `docs/strategies/redesign-2-core.md`.

## Notes

- 2026-11-10 [decision] The orchestrator reworded the CI Done when line: the Report shows the new job's steps, run as written in a scratch clone, failing on a throwaway commit that adds a path dependency on an old crate (and passing without it), instead of a real CI run.
  Reason: A real CI run needs a hand-pushed throwaway branch on origin; the merge policy forbids pushing branches by hand, and only the owner may delete remote branches.
  Rejected: Push a throwaway branch to origin and let CI fail on it
- 2026-11-10 [handoff] Escalated to the orchestrator after three Sol review rounds: rounds 1-3 recorded four fix_now findings (atelier-wvqx, atelier-6a6v, atelier-x7cu, atelier-86l6), all fixed with regression tests seen failing first; the last fix (dc635ae8 head, code at 3d21d702) has not been reviewed. Outcome, proof and open items are in the Report.

## Report

Every Done when line holds, at the commit under review. `next/` is a Cargo workspace of its own (`next/Cargo.toml`, `next/Cargo.lock`, every package version one the root lock already has), with `atelier-core` (domain values: `Item`, `Review`, `Finding`, `Status`, `Thread`/`Threads`, `Config`, and the open and settled rules) and `atelier-records` (split, parse, render, the `Store` over `.atelier/`, IDs, validation). Tests in `next/crates/atelier-records/tests/records.rs` and the crates' unit tests, 38 in all, pass with `cargo nextest run`: `each_record_kind_reads_and_writes_back_the_same_file` and `a_body_keeps_its_leading_and_trailing_newlines` (round trip, body byte for byte); `an_unknown_status_is_refused_naming_file_and_field`, `an_item_needs_goal_and_done_when`, `a_finding_needs_a_recommendation_as_its_first_line`, `an_approve_verdict_is_refused_while_its_file_holds_an_open_fix_now_finding`, `a_blocked_by_must_name_an_item` (each asserts the file and the field); `implementers_and_reviewers_are_lists_whose_last_entry_holds_the_role` with core's `the_last_thread_to_take_a_role_holds_it`; `findings_are_named_by_review_stem_and_number`, `a_finding_name_no_review_has_is_refused`, `a_finding_name_in_another_form_is_refused`; `caused_by_holds_an_item_id_or_a_commit_hash_checked_for_form_only`; `a_deferred_finding_is_settled_by_a_taken_or_dropped_line`; `a_fix_now_finding_stays_open_until_a_later_round_or_a_decision_closes_it` with core's open-rule tests; `creating_reviews_of_a_round_gives_each_its_own_name`; `config_reads_its_fields`, `config_refuses_what_it_does_not_know`, `a_config_error_names_the_line_of_the_key`. Each test was seen failing: a script applied 39 one-line mutations to the rules (raw-body split reverted, holder read as first, carried findings ignored, `deny_unknown_fields` removed, and so on) and each failed the tests named for it; making `ci_workflow` required fails the config test at compile time. CI's new `next` job (`.github/workflows/ci.yml`) runs `rm -rf ../crates`, `cargo fmt -- --check`, `cargo build --locked --all-targets` and `cargo nextest run --locked` in `next/`. Its `run:` steps, read from `ci.yml` and run each with `bash -e` in a scratch clone (`/home/supadava/.cache/atelier-fcb7/run_next_job.py`), passed on 3d21d702 (38 tests); on a throwaway commit that adds `atelier-workflow = { path = "../../../crates/atelier-workflow" }` to `atelier-records` and uses it, which builds with `crates/` present, they failed at the first cargo step with "failed to read `.../crates/atelier-workflow/Cargo.toml`: No such file or directory", and `cargo build --locked` alone failed the same way. `scripts/ci-jobs.sh` prints `next=true|false`; `scripts/ci-jobs.test.sh` passes its 24 cases, and fails 4 when next/ is dropped from the rules. `git diff --stat master...HEAD -- crates/` is empty. `scripts/count_non_test_rust.py HEAD next` counts the new workspace alone (1,197 lines: core 414, records 783), and its default output at 8f13d015 is still the recorded 19,433 with the same per-crate figures.

### Copied code

- `split_document` (`crates/atelier-records/src/lib.rs`) is `document::split`. Changed as the Scope says: it returns the body as the raw slice after the closing `---` line, where the old one trimmed its leading newlines. `document::join`, its inverse, is new.
- `record_id.rs` is `ids.rs`. Kept: `allocate_issue_id` as `allocate_id` (its `exists` callback returns `bool`, and errors are `io::Error`), `validate_record_id` as `validate_id`, `validate_project_slug` as `validate_slug`, `random_base36_suffix` as `random_suffix` (fixed at four characters, and it also names review files), `base36_padded` unchanged. Trimmed: `derive_project_slug` (for `init`, not built here), `legacy_issue_id` (test-only), the clock fallback when `/dev/urandom` cannot be read (it now fails instead), and `anyhow`.

### Deviations

- `count_non_test_rust.py` takes the directory as a second, optional argument after the revision (`count_non_test_rust.py HEAD next`), not as its only one: the scorecard on atelier-jah8's branch atelier-5fsj calls it as `count_non_test_rust.py <commit>` and reads `TOTAL:`, which still works. It now groups files by crate name, or by the top directory for anything outside a `crates/` directory, so `HEAD crates` lists the old crates and a whole-tree count shows `next` as its own line.
- `ci-jobs.sh`: the new job also runs when `rust-toolchain.toml`, `.cargo/` or `.github/workflows/` change, since each configures its build from outside `next/`. The old `rust` job no longer runs for a change only under `next/`, which it cannot read.

### Decisions

- The orchestrator reworded the CI Done when line (decision note above).
- A fix-now finding carried as `Now: kept` moves: the carrying finding becomes the open one and the earlier one is closed, so one problem is counted once across rounds. A carrying finding that says fixed, withdrawn or decided is not itself open. The spec said only "open until a later review marks it fixed or withdrawn".
- `Threads::take` does not append a thread that already holds the role, so an entry command run twice by the same thread is not recorded as a take-over.
- The Decisions entry format the spec left open: `### <date>: <question>`, then `Finding:`, `Answer by <thread>: <answer>` and `Outcome:` lines; an `Outcome:` needs a `Finding:` and an answer. Step C's `escalate` and `decide` may refine it.
- Review round 1 (Sol) found two holes, both fixed: a carrier kept as `Now: kept` with a weaker recommendation closed a fix-now finding unfixed, so such a carrier is now refused (atelier-wvqx; cases in `records_whose_names_or_lines_are_ambiguous_are_refused` and `an_approve_verdict_is_refused_while_its_file_holds_an_open_fix_now_finding`); and field lines inside a fenced example settled findings and answered decisions, so findings, Decisions and Responses now read only lines outside fences, through the reader the section splitter uses (atelier-6a6v; `field_lines_in_a_fenced_example_are_text`, and `leaves_out_fenced_lines_and_their_fences` in `document.rs`). Round 2 found that reader closed a four-backtick fence on a three-backtick line; a fence now closes as in CommonMark, only on a line of its own character, at least as long, with nothing after it (atelier-x7cu; the same two tests, with nested fences). Round 3 found a line of inline code such as ```` ```inline code``` ```` opened a fence that never closed and hid every later finding; as in CommonMark, a backtick fence's info string now cannot hold a backtick (atelier-86l6; `inline_code_on_a_line_of_its_own_opens_no_fence`). Each new test was seen failing on the code before its fix.
- `schema` and `schema_version` are typed fields of `Item`, `Review` and `Config` (`ItemSchema`, `ReviewSchema`, `SchemaVersion`), so front matter parses in one `serde_yaml::from_str` and its errors keep their field path (`status: unknown variant ...`); parsing through a `serde_yaml::Value` loses it.

### Differences corrected in the specification

In `docs/strategies/redesign-architecture.md`, Records: the one written form of front matter (field order, block style, `null`, RFC 3339 times with UTC as `Z`) and that the body is kept byte for byte; unknown keys refused; priority values; `blocked_by` names an item in the checkout; `implementers` appended unless the thread holds it; `caused_by` as an item ID or a 7 to 64 digit hex hash; the item's directory is its ID; sections as `##` headings, Goal and Done when required, `###` subsections, a Responses line may be a list item; the Decisions entry format; a review's `round` equals its file name's, suffixes base 36 and unique per item, finding numbers unique, `Was:` names an earlier round's finding with exactly one `Now:` or `Decided:`, every finding name must exist; the open rule with kept findings moving, and a kept carrier being fix now; fenced lines in a body are never read as headings or fields; approve refused while the file's own finding is open; what counts as deferred; `Taken:` names an item ID and a finding cannot have both lines; config's single values for `merge_strategy` and `provider`, and that `merge_check` and `forge` may be left out. In Workspace and crates: which paths besides `next/` run the new job.

### Open items

- `merge_strategy` has one possible value, `merge_commit`, so the field carries nothing; the spec could drop it (Sumeet's call, since it is a config option the spec names).
- A carried finding's `Was:` must name the latest finding of its chain for the open rule to work; a reviewer who names the round-1 finding in round 3 leaves round 2's kept finding open. The review brief (atelier-engh) should print the name to carry.
- Review files under a mission, and mission records, are step B. Reading records at a commit is the object reader's item; the parse and render functions take text, so it can feed them.
