---
created_at: "2026-11-10T19:33:55.713718555+00:00"
id: "atelier-jciw"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-jciw
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add `show`, `list` and `lead`, and missions in `status`, so none of today's nine work views is lost"
updated_at: "2026-11-10T20:41:28.634319437+00:00"
---

## Goal

Give a lead, a planner and Sumeet's agents every view of the work they have today, in two commands and `status`: `show` for one mission or item, `list` for many, and `lead` to take a mission and see what each item needs.

## Done when

- [x] `show <item>` prints the item in full: its status, holder, mission, labels, priority, blockers and who made it ready; its sections; and each review by round with its reviewer, verdict and findings, each finding by name with its recommendation and whether it is open. For a started item it reads the item's branch from wherever it is run, and it works for a done item.
- [x] `show <mission>` prints the mission on one page: its status and lead, its Outcome and success criteria, its items grouped by status with who holds each and what blocks each, the questions in its items' Decisions sections (unanswered first, then answered, one line each), and its plan reviews with their verdicts. It works for a done mission.
- [x] `list` with no flag prints all open work, grouped by mission and then by label, one line per item with its ID, status, priority, title and holder. `--ready` keeps items a thread could start now (ready, blockers done, mission ready); `--blocked` keeps items with a blocker that is not done and names it; `--needs-human` keeps items labelled `needs-human`; `--mission <id>` and `--label <l>` keep one mission's or one label's; `--mine` keeps what the calling thread holds as implementer, reviewer or lead; `--done` shows finished missions and items instead of open ones. Flags narrow together. A scenario test over one fixture checks each flag's output.
- [x] No view of today's nine `work` commands is lost. `docs/strategies/redesign-product.md` gains a table that maps each (`mission`, `missions` and `missions --all`, `ready`, `blocked`, `human`, `decisions`, `active`, `all`, `parent`) to the `show`, `list` or `status` output that replaces it, or says why it has no successor, and the scenario test above shows a finished mission, an answered question and a started item each appearing where the table says.
- [x] `lead <mission>`, on a ready mission, sets the caller as its lead, says which thread led it before if one did, and prints where each item stands and which need a thread, with a handoff line for each item that can be started now. It is refused on a mission that is not ready, naming `ready`. It makes no commit: it prints the path it wrote and a `Then:` line with the Git command that commits it.
- [x] `status` on the base, for a thread that holds nothing, shows the items that can be started, the missions in progress (a ready mission with a started item) and what waits on Sumeet (items labelled `needs-human`). For a mission's lead it says which mission it leads and what each of its items needs. A started item is shown with its status and holder, read from its branch, and never as ready.
- [x] The act table gains the row for `lead`, and the unit test over the table covers it.
- [x] `docs/strategies/redesign-architecture.md` matches what was built, and the flags of `list` leave its Open questions list.

## Scope

`show`, `list`, `lead` and the base's `status`, in `next/`. Everything but `lead` only reads. `list --deferred` is step C and `log` is mission 4. A dropped record is shown with its status, and its reason appears where `show` prints its Report or Result as text; the reader of the `Dropped:` line belongs to atelier-jq24, and this item does not need it. New code goes in new view modules in `atelier-app` and `atelier-cli`, since another implementer builds atelier-jq24 (`drop`, `close`, plan reviews) at the same time; keep additions to shared files (`main.rs`, `output.rs`, `acts.rs`) to what this item needs. No cache and no index: if reading started items from their branches proves slow, step E's rehearsal on real repositories is where it shows. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Status, Threads and roles and "Where records are read and written". Behaviour: `docs/strategies/redesign-product.md`, Commands (Anyone, Decider and lead), Handoff lines and "What `status` prints". The mapping line answers planning finding atelier-50br, which found that "no view lost" was promised more broadly than the listed flags delivered. Step A's fresh-agent runs (atelier-67nn, `evals/agent-factory/results/2026-11-10-next-loop.md`) set the wording conventions `lead`'s handoff lines follow: the child must be another thread, and a task an agent may paste into a shell has no backticks.

This is the third of step B's three items in `docs/strategies/redesign-2-core.md`.

## Report

Every Done when line holds. The views are covered by `atelier-cli`'s `tests/views.rs`, over one fixture (a ready mission in progress with a draft, a ready, a blocked and a started item, the last started by t1 in a second worktree; a done mission with a done item, an answered question and a plan review; a draft mission; an idle ready mission; a needs-human item, an item in no mission and a dropped one). `list` and each of its flags, narrowing together, and a finished mission under `--done`: `list_shows_open_work_by_mission_and_label_and_each_flag_narrows_it`, run from the base and from the second worktree; `--mine` for an implementer and a lead there, and for a round's reviewer in `list_mine_keeps_the_review_a_thread_holds`. `status` on the base with ready items, a started item shown with its holder from its branch (its base record says ready), the mission in progress and not the idle one, and the needs-human item apart: `status_on_the_base_shows_what_can_start_the_missions_in_progress_and_what_waits_on_a_human`. `lead` refused on a draft mission (naming `ready`), a done one, an item, and off the base; then on the ready mission: the previous lead, the items by status, one handoff for the one item that can start (not for an item made ready and not committed), the path written and the `Then:` commit line, nothing committed; the lead's `status`; a take-over naming the earlier lead; and a second run that writes nothing: `lead_takes_a_ready_mission_and_hands_each_item_that_can_start_to_another_thread`. `show` on a mission and a done mission, with questions unanswered first and the plan review: `show_prints_a_mission_on_one_page_with_its_items_questions_and_plan_reviews`; on a started item from the base and from its own worktree (where an uncommitted tick shows), a done item and a blocked one: `show_prints_a_started_item_from_its_branch_wherever_it_runs_and_a_done_one`; reviews by round with each finding's recommendation and whether it is open: `show_prints_each_review_by_round_with_its_findings_and_whether_each_is_open`. The act table test `every_act_in_the_table_from_every_status_sets_the_specified_status_or_is_refused` covers `lead` from each of a mission's statuses. The product specification's new table maps the nine views; the scenario tests show a finished mission under `list --done`, an answered question in `show <mission>` and a started item in `status` and `list`, as it says. All pass after the last change: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` in `next/` (152 tests), and the CI job's steps for `next/` in a scratch clone with `crates/` deleted. Each new test was seen failing under a deliberate breakage of what it covers (the lead row from draft, reading the base's record instead of the branch's, ready without its conditions, `--mine` without the reviewer, no unanswered-first order, needs-human not separated, in progress without a started item, `lead` rewriting for its own lead, plan reviews not read, a finding's open state inverted, `lead` reading items from the checkout).

### Decisions

See the decision notes: needs-human items wait apart in `status` and are not handed out by `lead`; the output says "a human", not a name; `lead` reads items as `status` does; how `list` groups and what `--done`, `--mine` and a bare mission mean.

### Surprises

- The handoff line is now one function, `output::handoff`, for `submit`'s reviewer and `lead`'s implementers; `submit`'s text is unchanged.
- `status` with about 30 started branches (31, debug build) takes 0.12 s, and so does `list`.

### Open items

- The product specification's Merge section says an approved item not yet merged shows "with how long it has been so" in `status`, `show` and the site. No record holds the time of a verdict, so `show` prints only "approved"; not built here, as no Done when line asks for it.
- `list` prints a holder only for a started item; a draft or ready one has none.

### Validation

The new binary driven by hand in a scratch repository (`next/target/debug/atelier`, shown as `atelier`; each line indented two spaces, so the tracker reads no heading in it): two missions besides a draft one, demo-m1ss ready and demo-m2dn done; items draft, ready, blocked, building (demo-d4dd, started by t1 in a second worktree), done and dropped; an unanswered question on demo-d4dd and an answered one on demo-e5ee; demo-f6ff labelled needs-human.

```text
  $ T3CODE_THREAD_ID=p1 atelier status
  You are thread p1, on the base branch main.
  Ready to start:
    demo-a1aa (high): Say hello
    demo-g7gg (medium): Add a --version flag
    demo-b2bb (medium): Say goodbye
      [ ] demo-a1aa is done (it is ready)
  Held by other threads:
    demo-d4dd (building, held by t1): Wave
  Missions in progress:
    demo-m1ss (ready, in progress): Greet everyone
  Waiting on a human (labelled needs-human):
    demo-f6ff (ready, medium, waits on a human): Choose a name for the bot
  Then: atelier start demo-a1aa
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier list
  Mission demo-m1ss (ready, in progress): Greet everyone
    No label:
      demo-b2bb (ready, medium, blocked by demo-a1aa, which is ready): Say goodbye
      demo-c3cc (draft, medium): Say hi
    Labelled ui:
      demo-a1aa (ready, high, can start now): Say hello
      demo-d4dd (building, medium, held by t1): Wave
  In no mission:
    Labelled cli:
      demo-g7gg (ready, medium, can start now): Add a --version flag
    Labelled needs-human:
      demo-f6ff (ready, medium, waits on a human): Choose a name for the bot
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier list --ready
  Mission demo-m1ss (ready, in progress): Greet everyone
    Labelled ui:
      demo-a1aa (ready, high, can start now): Say hello
  In no mission:
    Labelled cli:
      demo-g7gg (ready, medium, can start now): Add a --version flag
    Labelled needs-human:
      demo-f6ff (ready, medium, waits on a human): Choose a name for the bot
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier list --blocked
  Mission demo-m1ss (ready, in progress): Greet everyone
    No label:
      demo-b2bb (ready, medium, blocked by demo-a1aa, which is ready): Say goodbye
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier list --needs-human
  In no mission:
    Labelled needs-human:
      demo-f6ff (ready, medium, waits on a human): Choose a name for the bot
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier list --mission demo-m1ss
  Mission demo-m1ss (ready, in progress): Greet everyone
    No label:
      demo-b2bb (ready, medium, blocked by demo-a1aa, which is ready): Say goodbye
      demo-c3cc (draft, medium): Say hi
    Labelled ui:
      demo-a1aa (ready, high, can start now): Say hello
      demo-d4dd (building, medium, held by t1): Wave
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier list --label ui
  Mission demo-m1ss (ready, in progress): Greet everyone
    Labelled ui:
      demo-a1aa (ready, high, can start now): Say hello
      demo-d4dd (building, medium, held by t1): Wave
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier list --done
  Mission demo-m2dn (done): Set up the project
    No label:
      demo-e5ee (done, low): Create the repository
  In no mission:
    No label:
      demo-h8hh (dropped, medium): Port to Windows
  (exit 0)
  $ T3CODE_THREAD_ID=t1 atelier list --mine
  Mission demo-m1ss (ready, in progress): Greet everyone
    Labelled ui:
      demo-d4dd (building, medium, held by t1): Wave
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier show demo-m1ss
  Mission demo-m1ss: Greet everyone
  Status: ready, in progress
  Lead: nobody
  Labels: none
  Made ready by sumeet at 2026-11-10 09:00:00 +00:00

  ## Outcome

  Every greeting works.

  ## Success criteria

  - hello: `hello` prints hello.

  Items:
    Draft:
      demo-c3cc (medium): Say hi
    Ready:
      demo-a1aa (high, can start now): Say hello
      demo-b2bb (medium, blocked by demo-a1aa, which is ready): Say goodbye
    Building:
      demo-d4dd (medium, held by t1): Wave

  Questions:
    demo-d4dd: 2026-11-10: Should waving be animated? (unanswered)

  Plan reviews:
    None.
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier show demo-m2dn
  Mission demo-m2dn: Set up the project
  Status: done
  Lead: nobody
  Labels: none
  Made ready by sumeet at 2026-11-10 09:00:00 +00:00

  ## Outcome

  A project exists.

  ## Result

  It exists, at commit abc.

  Items:
    Done:
      demo-e5ee (low): Create the repository

  Questions:
    demo-e5ee: 2026-11-09: Which license? Answer by lead1: MIT.

  Plan reviews:
    None.
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier show demo-d4dd
  Item demo-d4dd: Wave
  Status: building, held by t1
  Mission: demo-m1ss (ready)
  Labels: ui
  Priority: medium
  Blocked by: none
  Made ready by sumeet at 2026-11-10 09:00:00 +00:00

  ## Goal

  Do demo-d4dd.

  ## Done when

  - [ ] It is done.

  ## Decisions

  ### 2026-11-10: Should waving be animated?

  Reviews: none yet.
  (exit 0)
  $ T3CODE_THREAD_ID=p1 atelier lead demo-m2dn
  `atelier lead demo-m2dn` refused; nothing changed.
  Mission demo-m2dn is done, and `atelier lead` runs on a mission that is ready. Nothing is left to lead on it.
  (exit 1)
  $ T3CODE_THREAD_ID=lead1 atelier lead demo-m1ss
  You now lead mission demo-m1ss: Greet everyone
  Nobody led it before.
  Its items:
    Draft:
      demo-c3cc (medium): Say hi
    Ready:
      demo-a1aa (high, can start now): Say hello
      demo-b2bb (medium, blocked by demo-a1aa, which is ready): Say goodbye
    Building:
      demo-d4dd (medium, held by t1): Wave
  An implementer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
    Run atelier start demo-a1aa and follow what it prints.
  Then wait for it to finish, and run `atelier status`.
  Wrote, and did not commit:
    .atelier/missions/demo-m1ss/mission.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/missions/demo-m1ss/mission.md && git commit -m "Lead demo-m1ss" -- .atelier/missions/demo-m1ss/mission.md
  (exit 0)
  $ git add -- .atelier/missions/demo-m1ss/mission.md && git commit -m "Lead demo-m1ss" -- .atelier/missions/demo-m1ss/mission.md
  $ T3CODE_THREAD_ID=lead1 atelier status
  You are thread lead1, on the base branch main.
  Ready to start:
    demo-a1aa (high): Say hello
    demo-g7gg (medium): Add a --version flag
    demo-b2bb (medium): Say goodbye
      [ ] demo-a1aa is done (it is ready)
  Held by other threads:
    demo-d4dd (building, held by t1): Wave
  Missions in progress:
    demo-m1ss (ready, in progress, led by lead1): Greet everyone
  Waiting on a human (labelled needs-human):
    demo-f6ff (ready, medium, waits on a human): Choose a name for the bot
  You lead mission demo-m1ss: Greet everyone
    Draft:
      demo-c3cc (medium): Say hi
    Ready:
      demo-a1aa (high, can start now): Say hello
      demo-b2bb (medium, blocked by demo-a1aa, which is ready): Say goodbye
    Building:
      demo-d4dd (medium, held by t1): Wave
  An implementer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
    Run atelier start demo-a1aa and follow what it prints.
  Then wait for it to finish, and run `atelier status`.
  (exit 0)
  $ T3CODE_THREAD_ID=lead1 atelier list --mine
  Mission demo-m1ss (ready, in progress, led by lead1): Greet everyone
    No label:
      demo-b2bb (ready, medium, blocked by demo-a1aa, which is ready): Say goodbye
      demo-c3cc (draft, medium): Say hi
    Labelled ui:
      demo-a1aa (ready, high, can start now): Say hello
      demo-d4dd (building, medium, held by t1): Wave
  (exit 0)
```

## Notes

- 2026-11-10 [decision] Expected size: about 650 non-test lines of Rust, about 600 of tests and about 50 changed lines of the two specifications; escalate past about twice that
  Reason: The orchestrator's brief gives this estimate
- 2026-11-10 [decision] Items labelled needs-human that nobody has started are listed in status under 'Waiting on a human', not under 'Ready to start', and lead hands none of them to a thread; list --ready keeps them, marked 'waits on a human'
  Reason: The product specification makes such an item a question for the human, which an agent closes with his answer, so pointing an agent at it with a Then line would start the wrong work; list --ready keeps the Done when line's literal rule (ready, blockers done, mission ready), and its flags narrow together, so --ready --needs-human still finds them
  Rejected: Offer them as ready like any other item
- 2026-11-10 [decision] The output says 'Waiting on a human', not the owner's name
  Reason: The product specification's 'waits on Sumeet' names the one user it serves; the binary's text and its tests name no person
- 2026-11-10 [decision] lead reads its items and the missions as status does, from the base's last commit and each started item's branch, and reads and writes only its own mission in the checkout of the base
  Reason: What lead says can start now must be what start will start, and start reads the base's last commit; an item made ready and not yet committed would otherwise be handed to a thread that start refuses. The architecture specification lists lead among the commands that read this way.
  Rejected: Read every record from the checkout, as new, set and ready do
- 2026-11-10 [decision] list shows an item once, under its sorted set of labels; --done means done or dropped; --mine keeps a mission the caller leads with its items; a mission with no item kept is shown unless --ready, --blocked or --needs-human is given
  Reason: Listing an item under each of its labels, as today's views do, repeats it and miscounts; dropped work is finished as much as done work; a lead's own work is its mission's items; a mission heading alone answers 'what missions are there' (today's work missions) but says nothing to an item-only question
