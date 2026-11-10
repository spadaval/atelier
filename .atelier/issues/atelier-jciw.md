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
updated_at: "2026-11-10T20:24:48.373440144+00:00"
---

## Goal

Give a lead, a planner and Sumeet's agents every view of the work they have today, in two commands and `status`: `show` for one mission or item, `list` for many, and `lead` to take a mission and see what each item needs.

## Done when

- [ ] `show <item>` prints the item in full: its status, holder, mission, labels, priority, blockers and who made it ready; its sections; and each review by round with its reviewer, verdict and findings, each finding by name with its recommendation and whether it is open. For a started item it reads the item's branch from wherever it is run, and it works for a done item.
- [ ] `show <mission>` prints the mission on one page: its status and lead, its Outcome and success criteria, its items grouped by status with who holds each and what blocks each, the questions in its items' Decisions sections (unanswered first, then answered, one line each), and its plan reviews with their verdicts. It works for a done mission.
- [ ] `list` with no flag prints all open work, grouped by mission and then by label, one line per item with its ID, status, priority, title and holder. `--ready` keeps items a thread could start now (ready, blockers done, mission ready); `--blocked` keeps items with a blocker that is not done and names it; `--needs-human` keeps items labelled `needs-human`; `--mission <id>` and `--label <l>` keep one mission's or one label's; `--mine` keeps what the calling thread holds as implementer, reviewer or lead; `--done` shows finished missions and items instead of open ones. Flags narrow together. A scenario test over one fixture checks each flag's output.
- [ ] No view of today's nine `work` commands is lost. `docs/strategies/redesign-product.md` gains a table that maps each (`mission`, `missions` and `missions --all`, `ready`, `blocked`, `human`, `decisions`, `active`, `all`, `parent`) to the `show`, `list` or `status` output that replaces it, or says why it has no successor, and the scenario test above shows a finished mission, an answered question and a started item each appearing where the table says.
- [ ] `lead <mission>`, on a ready mission, sets the caller as its lead, says which thread led it before if one did, and prints where each item stands and which need a thread, with a handoff line for each item that can be started now. It is refused on a mission that is not ready, naming `ready`. It makes no commit: it prints the path it wrote and a `Then:` line with the Git command that commits it.
- [ ] `status` on the base, for a thread that holds nothing, shows the items that can be started, the missions in progress (a ready mission with a started item) and what waits on Sumeet (items labelled `needs-human`). For a mission's lead it says which mission it leads and what each of its items needs. A started item is shown with its status and holder, read from its branch, and never as ready.
- [ ] The act table gains the row for `lead`, and the unit test over the table covers it.
- [ ] `docs/strategies/redesign-architecture.md` matches what was built, and the flags of `list` leave its Open questions list.

## Scope

`show`, `list`, `lead` and the base's `status`, in `next/`. Everything but `lead` only reads. `list --deferred` is step C and `log` is mission 4. A dropped record is shown with its status, and its reason appears where `show` prints its Report or Result as text; the reader of the `Dropped:` line belongs to atelier-jq24, and this item does not need it. New code goes in new view modules in `atelier-app` and `atelier-cli`, since another implementer builds atelier-jq24 (`drop`, `close`, plan reviews) at the same time; keep additions to shared files (`main.rs`, `output.rs`, `acts.rs`) to what this item needs. No cache and no index: if reading started items from their branches proves slow, step E's rehearsal on real repositories is where it shows. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Status, Threads and roles and "Where records are read and written". Behaviour: `docs/strategies/redesign-product.md`, Commands (Anyone, Decider and lead), Handoff lines and "What `status` prints". The mapping line answers planning finding atelier-50br, which found that "no view lost" was promised more broadly than the listed flags delivered. Step A's fresh-agent runs (atelier-67nn, `evals/agent-factory/results/2026-11-10-next-loop.md`) set the wording conventions `lead`'s handoff lines follow: the child must be another thread, and a task an agent may paste into a shell has no backticks.

This is the third of step B's three items in `docs/strategies/redesign-2-core.md`.
