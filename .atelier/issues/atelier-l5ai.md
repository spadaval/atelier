---
created_at: "2026-11-11T05:22:58.125720072+00:00"
id: "atelier-l5ai"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-l5ai
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1ie1"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-psjw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add the six small views the Desk needs to atelier-app: read-only opening from commits, the mission and item views, documents, approval time and plan changes"
updated_at: "2026-11-11T05:39:07.732202217+00:00"
---

## Goal

Give `atelier-app` the six small views the Desk's pages need and the core lacks, so the Desk's server (atelier-uwxy) reads a repository from commits only and shows what `atelier show` shows without copying CLI code. They are M1, M2, M3, M4, M6 and M7 in `docs/architecture/desk.md`, Missing views. Each is a move or an exposure of code that exists; the command line calls each view it moves, so there is still one copy.

## Done when

- [x] **M1, a repository opens for reading from commits only.** `atelier-app` has a way to open a repository by path and caller that reads `config.toml` from the commit `HEAD` names, and through which every item and mission record is read from a commit, never from the opened checkout's files. A test opens a scratch repository whose checkout has an uncommitted edit to `.atelier/config.toml` and to one started item's `item.md`, and reads the committed values of both. The acts and the CLI's commands keep opening as they do today.
- [x] **M3, the mission view is in `atelier-app`.** One function returns what `show <mission>` prints as data: the mission, whether it is in progress, its entries, its open and answered questions with the item each is on, and its plan reviews. The CLI's `show <mission>` calls it, and its existing tests pass unchanged.
- [x] **M7, the item view is in `atelier-app`.** One function returns what `show <item>` prints as data, and says whether the record came from the base or from the item's branch. The CLI's `show <item>` calls it, and its existing tests pass unchanged.
- [x] **M2, a mission's documents can be listed at a commit.** A public function of `atelier-records` or `atelier-app` returns the paths of the files of `.atelier/missions/<id>/` at a commit, without `mission.md` and anything under `reviews/`. A test covers a mission with a `design.md`, a `notes.md` and a review.
- [x] **M4, an approved item's approval time is given by `atelier-app`.** The code that fills `Entry::approved` (today `annotate` and `git::approved_at` in `atelier-cli`) lives in `atelier-app` and the CLI calls it there. Each function still lives in one crate only.
- [x] **M6, a plan's changes since a reviewed commit are read from commits.** A function returns the paths of a mission's plan that differ between a given commit and the base's tip, comparing the two commits and not the checkout's files. A test covers a plan file changed after the reviewed commit, and one unchanged.
- [x] No command's output or refusal changes: `cargo nextest run -j 4` in `next/` passes with no existing test edited except to follow a moved function's path, and the Report lists any such edit.
- [x] `docs/architecture/desk.md` marks M1, M2, M3, M4, M6 and M7 with the function that now gives each, in place of "missing".

## Scope

`next/crates/atelier-app`, `next/crates/atelier-records`, the call sites in `next/crates/atelier-cli`, and `docs/architecture/desk.md`. No new crate and no new dependency. M5, the history of acts, is not in this item or this mission (decision note of 2026-11-11 on atelier-z4cr). Do not change what any command prints or refuses, and do not change the specifications in `docs/strategies/redesign-*.md`.

Expected size: about 250 lines of code moved or added and about 200 of tests, in under 15 files.

## Context

- The contract and the list of views: `docs/architecture/desk.md`. The strategy: `docs/strategies/desk.md` (Shape: "The tracker changes where the Desk needs it to"; Authority).
- This item starts only after atelier-psjw has merged, since it builds on the `Repo` that item moves into `atelier-app`. A `Repo` there holds `&Skip` and is not `Send`; whether the read-only opening is a second constructor of `Repo` or a type of its own is the implementer's choice, said in the Report.
- The switch (atelier-u1ko) moves `next/crates/` to `crates/`. Before starting, check that neither atelier-u1ko nor atelier-wd1i is in progress. If the switch merges while this branch is open, bring master in with `atelier merge --sync`.
- Sizes: complexity low, value high, risk low. The risk is a moved function that changes a command's output; the unchanged CLI tests guard it.

## Notes

- 2026-11-11 [note] Handoff: approved by sol-l5ai-r1 (atelier-d65y) at 36bbd029 in one round with no findings; see the Report. Not merged: the orchestrator merges.

## Report

All eight Done when lines hold. The new tests are in `next/crates/atelier-app/tests/views.rs` and one in `next/crates/atelier-records/tests/at_commit.rs`; `cargo nextest run -j 4` in `next/` passes 253 of 253 (247 before), with no existing test edited.

- M1: `Repo::open_committed(dir, caller, skip)`. `a_repository_opened_from_commits_reads_no_uncommitted_record` edits `config.toml` (slug) and the started item's `item.md` (title) without committing, with the item's branch checked out: `Repo::open` reads the edits, `open_committed` reads the committed values through `locate`, `everything` and `current`. `the_config_is_read_from_the_commit_head_names` detaches `HEAD` behind the base and reads `HEAD`'s config. Removing either half of the change makes the first test fail (checked by mutation). `Repo::open` is unchanged, so the CLI and the acts open as before.
- M3: `atelier_app::mission_view(&mut repo, id) -> Result<Option<MissionView>>`. `show <mission>` calls it. `the_mission_view_holds_what_show_prints`; the CLI's `show_prints_a_mission_on_one_page_with_its_items_questions_and_plan_reviews` passes unchanged.
- M7: `atelier_app::item_view(&mut repo, id) -> Result<Option<ItemView>>`, with `ItemView::read_from` and `Located::read_from` giving `ReadFrom::Base` or `ReadFrom::Branch`. `show <item>` calls it. `the_item_view_says_which_record_it_read`; the CLI's `show_prints_a_started_item_from_its_branch_wherever_it_runs_and_a_done_one`, `show_prints_each_review_by_round_with_its_findings_and_whether_each_is_open` and the settlement checks in `tests/deferred.rs` pass unchanged.
- M2: `CommitReader::mission_documents(commit, id)`. `lists_a_missions_documents_without_its_record_or_reviews` covers `design.md`, `notes.md`, a nested `notes/spike.md` and a review, and an uncommitted file that is not listed.
- M4: `annotate`, `annotate_all` and `git::approved_at` moved from `atelier-cli` to `atelier-app` and exist there only; the CLI imports them. The approval time is still checked by the CLI's `a_missions_lead_decides_its_items_questions_and_sees_each_and_since_when_one_is_approved` and `an_approved_item_merges_in_one_commit_that_marks_it_done`.
- M6: `atelier_app::plan::plan_changed_at(&mut repo, id, commit, tip) -> Result<Option<Vec<String>>>`. `plan_changes_are_read_between_two_commits` covers a design changed after the reviewed commit (listed), notes unchanged in commits and edited in the checkout (not listed), a new plan review (not listed), and a commit the clone lacks (`None`). `plan_changed` and `plan_changed_at` share one filter, `in_plan`.
- `docs/architecture/desk.md` names the function behind each of the six, in its tables and in Missing views; M5 stays missing.

### Decisions

- The read-only opening is a second constructor of `Repo`, not its own type, so the views, `links` and `facts` serve both. A private flag makes `locate` read a checked-out item branch from its tip. `Repo` is still not `Send` (it holds `&Skip`). Planning's and the acts' methods (`store`, `checkout_item`, `locate_planned`, `planned`, `new_id`, `uncommitted`) still read the checkout; the constructor's documentation says they are not for a repository opened this way.
- `item_view` shows a settlement the checkout holds uncommitted, as `show` does, except in a repository opened with `open_committed`.

### Surprises

- `config.toml` at `HEAD` is what the Done when asks for, but a detached checkout far behind the base, like this host's main checkout, still gives that old commit's config. The document now says the server opens `git::checkout_of(root, base)` where there is one.
