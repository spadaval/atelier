---
created_at: "2026-11-10T00:56:07.800857010+00:00"
id: "atelier-yptz"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-yptz
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-67nn"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-10T20:18:31.300102594+00:00"
status: "done"
title: "Add mission records and the commands that write a plan: `new`, `set`, `ready` and `start --new`"
updated_at: "2026-11-10T20:18:31.300102594+00:00"
---

## Goal

Let a planner write work down and say go on the new records: a mission record, the commands that create and change drafts, and `ready`. With them `start` can check everything the act table asks of it, and work Sumeet asks for directly can begin with `start --new`.

## Done when

- [x] `missions/<id>/mission.md` reads, writes and validates as the architecture specification's Records section defines it, with its body kept byte for byte. Validation names the file and the field for an unknown status and for a mission with no Outcome section. Any other file in a mission's directory is not examined: a test puts a Markdown file with no front matter and a file that is not text beside `mission.md`, and the mission still validates. An item whose `mission` names no mission is rejected, naming the item and the field.
- [x] `new "<title>"` on the base writes `items/<id>/item.md` as a draft with empty Goal, Done when and Scope sections and prints its path. `--in <mission>` sets its mission and is refused for a mission that does not exist. `--mission` writes `missions/<id>/mission.md` as a draft with its section headings instead. An ID is the config's slug and four characters, and no mission and item share one.
- [x] `set <id>` changes a draft or ready item's title, priority, labels, mission, blocked-by and caused-by, and a draft or ready mission's title and labels, which are the fields the architecture specification gives each record. It changes nothing else in the file: a test compares the body byte for byte. It is refused, with nothing written, for an item-only field on a mission, a blocked-by that names no item, a mission that names no mission, a caused-by that is neither an item ID nor a commit hash in form, a started item (naming its branch as where its record now changes), and a done or dropped record.
- [x] `ready <id>... --by <name>` sets each named draft item or mission to `ready` with who said so and when. If any named ID is unknown or not a draft, nothing is written and each such ID is named. `--by` is required. Readying a mission changes no item.
- [x] `new`, `set` and `ready` make no commit. Each prints the paths it wrote and ends with a `Then:` line giving the Git command that commits them. `status` on the base lists record changes not yet committed.
- [x] `start` checks every condition of its row in the act table, each with a test of its refusal and the way out it names: a blocker that is not done; a mission that is not ready, naming `ready`; and an item that is ready in the base checkout but not in the base's last commit, saying the change is not committed yet.
- [x] `start --new "<title>"` creates an item already `building` on a new branch named for it, with the caller as its implementer, in one commit made the way `start` makes its own, so an interrupted run leaves no branch. `status` then tells the implementer to write the item's Goal and Done when lines first. That is a line of orientation and not a condition: `submit` checks what its row in the act table lists and nothing more.
- [x] The act table gains the rows for `ready` on an item, `ready` on a mission and `start --new`, and the unit test over the table covers them from every status.
- [x] `docs/strategies/redesign-architecture.md` matches what was built: the mission record, the three rows, where planning records are read (the checkout by the planning commands, the base's last commit by `start`), and the flags of `set`, which leaves the Open questions list.

## Scope

Mission records and `new`, `set`, `ready` and `start --new` in `next/`. `drop`, `close` and plan reviews are atelier-jq24; `show`, `list` and `lead` are atelier-jciw; `new --from <finding>` is step C; the Git hooks are step D, so nothing validates a hand-made commit yet and each command validates what it writes. Nothing is copied from the old workspace: ID generation is already in `atelier-records`. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records, Status and "Where records are read and written". Behaviour: `docs/strategies/redesign-product.md`, Lifecycle, Commands (Planner, Implementer) and "Where records change". The specifications say planning is committed and pushed with Git and that no act makes a commit only to change a status, which is why these commands write and do not commit. Step A's fresh-agent runs (atelier-67nn, `evals/agent-factory/results/2026-11-10-next-loop.md`) set the conventions for what commands print: a `Then:` line naming the next command, and no backticks around a command an agent may paste into a shell.

This is the first of step B's three items in `docs/strategies/redesign-2-core.md`.

## Report

Every Done when line holds. Mission records read, write back byte for byte and validate (`atelier-records` tests `each_record_kind_reads_and_writes_back_the_same_file`, `a_mission_reads_its_fields_and_keeps_its_body`, `a_mission_with_an_unknown_status_is_refused_naming_file_and_field`, `a_mission_needs_an_outcome_section`, `a_missions_other_files_are_not_examined`, `an_items_mission_must_name_a_mission`, and `reads_a_mission_as_a_commit_holds_it_and_none_where_it_has_none`). `new`, `set` and `ready` are covered by `atelier-cli`'s `tests/planning.rs`: `new_writes_a_draft_item_or_mission_commits_nothing_and_says_how_to_commit`, `new_in_a_mission_that_does_not_exist_is_refused_and_writes_nothing`, `the_planning_commands_run_only_on_the_base_branch`, `set_changes_each_named_field_and_nothing_else_in_the_file`, `set_is_refused_with_nothing_written_for_a_field_or_record_it_cannot_change`, `ready_marks_each_named_draft_with_who_and_when_and_changes_no_other_item`, `ready_writes_nothing_when_any_named_id_is_unknown_or_not_a_draft` and `status_on_the_base_lists_planning_not_yet_committed`. `start`'s conditions: a blocker not done and an existing branch keep step A's `start_refuses_a_ready_item_whose_blocker_is_not_done_or_whose_branch_exists`; the mission is `start_is_refused_until_the_items_mission_is_ready_and_names_ready` here and `start_needs_the_items_mission_ready_on_the_base` in `atelier-app`; readiness not committed is `start_says_when_an_item_or_its_mission_is_ready_in_the_base_checkout_and_not_committed`. `start --new` is `start_new_creates_an_item_building_on_its_own_branch_in_one_commit` (which also submits an item with an empty Done when, so the orientation line is no condition) and `a_start_new_stopped_before_its_branch_leaves_no_branch_and_no_item`. The act table test `every_act_in_the_table_from_every_status_sets_the_specified_status_or_is_refused` covers `ready` on an item and `start --new` from all eight statuses and `ready` on a mission from all four of a mission's. The printed commit commands work from a subdirectory and with paths a shell would split (`the_printed_commit_works_from_a_subdirectory_and_with_paths_a_shell_would_split`). All pass: `cargo nextest run` in `next/`, 145 tests, after the last change; each new test was seen failing under a deliberate breakage of the behaviour it covers.

### Decisions

- A mission's status is its own type, `MissionStatus` (draft, ready, done, dropped), so a mission record naming an item's status such as `building` is refused when read, with the same "status: unknown variant" message an item gets. The act table stays in item statuses; `MissionStatus::status` maps across, and `acts::check_mission` checks a mission's row.
- `new`, `set` and `ready` run only on the base branch and are refused elsewhere, naming the checkout that has the base: the product specification files them under "Planner, on the base" and says planning happens there.
- A planning command validates what it writes with `validate` over every record of the checkout, with its records swapped in, and refuses on a problem in a file it writes. One mechanism gives the refusals for a blocked-by, a mission and a caused-by, worded as validation words them, naming the file and the field.
- `start`'s "not committed yet" is said in place of the usual refusal when the item, or the mission its base record names, is ready in the checkout that has the base and not at the base's tip, and it gives the command that commits all of that checkout's uncommitted planning.
- `set`'s flags: `--title`, `--priority`, `--labels a,b`, `--mission <id>`, `--blocked-by a,b`, `--caused-by <item or commit>`, at least one; a list flag replaces the list and an empty value clears. A new item's priority is `medium`.
- `Act` gains `Ready`, `ReadyMission` and `StartNew`, and `Role` gains `Planner`. No thread holds the planner's role, so `check` skips the role refusal and the holder check for it.

### Responses

atelier-ak29: fixed in ad69c6c5
atelier-kupf: fixed in ad69c6c5

### Open items

- No mission and item share an ID because `Repo::new_id` passes every item and mission ID in the checkout and at the base's tip, and every local branch, to `allocate_id`; read, not tested, since a test cannot force the random suffix to collide.
- The product specification says `ready` prints whether the plan was reviewed and changed since. Plan reviews are atelier-jq24's, so `ready` prints neither yet.
- Non-test Rust in `next/` went from 5,273 to 6,288 lines (`scripts/count_non_test_rust.py`), above the brief's expected 600.

### Validation

The new binary driven by hand in a scratch repository (`next/target/debug/atelier`, shown as `atelier`; `T3CODE_THREAD_ID=planner`, then `impl` from the first `start`):

```text
$ atelier new --mission "Greet everyone"
Created draft mission demo-gicu, with empty sections to write.
Its items are created with: atelier new "<title>" --in demo-gicu
A planner makes it ready with: atelier ready demo-gicu --by <name>
Wrote, and did not commit:
  .atelier/missions/demo-gicu/mission.md
Planning counts once it is committed on the base.
Then: git add -- .atelier/missions/demo-gicu/mission.md && git commit -m "Plan mission demo-gicu" -- .atelier/missions/demo-gicu/mission.md
(exit 0)
$ atelier new "Say hello" --in demo-gicu
Created draft item demo-x57r, with empty Goal, Done when and Scope sections to write.
A planner makes it ready with: atelier ready demo-x57r --by <name>
Wrote, and did not commit:
  .atelier/items/demo-x57r/item.md
Planning counts once it is committed on the base.
Then: git add -- .atelier/items/demo-x57r/item.md && git commit -m "Plan demo-x57r" -- .atelier/items/demo-x57r/item.md
(exit 0)
$ atelier set demo-x57r --priority high --labels cli,greeting
Set the fields of item demo-x57r.
Wrote, and did not commit:
  .atelier/items/demo-x57r/item.md
Planning counts once it is committed on the base.
Then: git add -- .atelier/items/demo-x57r/item.md && git commit -m "Set fields of demo-x57r" -- .atelier/items/demo-x57r/item.md
(exit 0)
$ atelier set demo-gicu --priority high
`atelier set demo-gicu` refused; nothing changed.
demo-gicu is a mission, and --priority only an item has. A mission's fields are --title and --labels.
(exit 1)
$ atelier ready demo-gicu demo-x57r --by sumeet
Made ready, as sumeet said: demo-gicu, demo-x57r.
Wrote, and did not commit:
  .atelier/items/demo-x57r/item.md
  .atelier/missions/demo-gicu/mission.md
Planning counts once it is committed on the base.
Then: git add -- .atelier/items/demo-x57r/item.md .atelier/missions/demo-gicu/mission.md && git commit -m "Ready demo-gicu, demo-x57r" -- .atelier/items/demo-x57r/item.md .atelier/missions/demo-gicu/mission.md
(exit 0)
$ atelier status
You are thread planner, on the base branch main.
Planning not committed yet, which `atelier start` does not read until it is: .atelier/items/demo-x57r/item.md, .atelier/missions/demo-gicu/mission.md
Commit it with: git add -- .atelier/items/demo-x57r/item.md .atelier/missions/demo-gicu/mission.md && git commit -m "Plan" -- .atelier/items/demo-x57r/item.md .atelier/missions/demo-gicu/mission.md
Nothing is ready to start.
(exit 0)
$ atelier start demo-x57r
`atelier start demo-x57r` refused; nothing changed.
demo-x57r is ready in this checkout, but not in main's last commit, which is what `atelier start` reads: the change is not committed yet.
Commit the planning with: git add -- .atelier/items/demo-x57r/item.md .atelier/missions/demo-gicu/mission.md && git commit -m "Plan" -- .atelier/items/demo-x57r/item.md .atelier/missions/demo-gicu/mission.md
Then: atelier start demo-x57r
(exit 1)
$ git add -- .atelier/items/demo-x57r/item.md .atelier/missions/demo-gicu/mission.md && git commit -m "Plan" -- .atelier/items/demo-x57r/item.md .atelier/missions/demo-gicu/mission.md
(exit 0)
$ atelier start demo-x57r
Started demo-x57r on branch demo-x57r.

You are the implementer of demo-x57r (thread impl).
Item: .atelier/items/demo-x57r/item.md
Title: Say hello
Status: building
Write its Goal and its Done when lines in .atelier/items/demo-x57r/item.md first: they say what the change must do, and the reviewer judges it against them.
To submit it for review:
  [ ] Report: written under `## Report`
`atelier submit` commits only the item's directory, so commit your change first.
Then: atelier submit
(exit 0)
$ git switch main
$ atelier start --new "Fix the greeting"
Created demo-wy29 and started it on branch demo-wy29.

You are the implementer of demo-wy29 (thread impl).
Item: .atelier/items/demo-wy29/item.md
Title: Fix the greeting
Status: building
Write its Goal and its Done when lines in .atelier/items/demo-wy29/item.md first: they say what the change must do, and the reviewer judges it against them.
To submit it for review:
  [ ] Report: written under `## Report`
`atelier submit` commits only the item's directory, so commit your change first.
Then: atelier submit
(exit 0)
```
