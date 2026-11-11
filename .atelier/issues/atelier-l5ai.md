---
created_at: "2026-11-11T05:22:58.125720072+00:00"
id: "atelier-l5ai"
issue_type: "task"
labels: []
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
status: "backlog"
title: "Add the six small views the Desk needs to atelier-app: read-only opening from commits, the mission and item views, documents, approval time and plan changes"
updated_at: "2026-11-11T05:22:58.125720072+00:00"
---

## Goal

Give `atelier-app` the six small views the Desk's pages need and the core lacks, so the Desk's server (atelier-uwxy) reads a repository from commits only and shows what `atelier show` shows without copying CLI code. They are M1, M2, M3, M4, M6 and M7 in `docs/architecture/desk.md`, Missing views. Each is a move or an exposure of code that exists; the command line calls each view it moves, so there is still one copy.

## Done when

- [ ] **M1, a repository opens for reading from commits only.** `atelier-app` has a way to open a repository by path and caller that reads `config.toml` from the commit `HEAD` names, and through which every item and mission record is read from a commit, never from the opened checkout's files. A test opens a scratch repository whose checkout has an uncommitted edit to `.atelier/config.toml` and to one started item's `item.md`, and reads the committed values of both. The acts and the CLI's commands keep opening as they do today.
- [ ] **M3, the mission view is in `atelier-app`.** One function returns what `show <mission>` prints as data: the mission, whether it is in progress, its entries, its open and answered questions with the item each is on, and its plan reviews. The CLI's `show <mission>` calls it, and its existing tests pass unchanged.
- [ ] **M7, the item view is in `atelier-app`.** One function returns what `show <item>` prints as data, and says whether the record came from the base or from the item's branch. The CLI's `show <item>` calls it, and its existing tests pass unchanged.
- [ ] **M2, a mission's documents can be listed at a commit.** A public function of `atelier-records` or `atelier-app` returns the paths of the files of `.atelier/missions/<id>/` at a commit, without `mission.md` and anything under `reviews/`. A test covers a mission with a `design.md`, a `notes.md` and a review.
- [ ] **M4, an approved item's approval time is given by `atelier-app`.** The code that fills `Entry::approved` (today `annotate` and `git::approved_at` in `atelier-cli`) lives in `atelier-app` and the CLI calls it there. Each function still lives in one crate only.
- [ ] **M6, a plan's changes since a reviewed commit are read from commits.** A function returns the paths of a mission's plan that differ between a given commit and the base's tip, comparing the two commits and not the checkout's files. A test covers a plan file changed after the reviewed commit, and one unchanged.
- [ ] No command's output or refusal changes: `cargo nextest run -j 4` in `next/` passes with no existing test edited except to follow a moved function's path, and the Report lists any such edit.
- [ ] `docs/architecture/desk.md` marks M1, M2, M3, M4, M6 and M7 with the function that now gives each, in place of "missing".

## Scope

`next/crates/atelier-app`, `next/crates/atelier-records`, the call sites in `next/crates/atelier-cli`, and `docs/architecture/desk.md`. No new crate and no new dependency. M5, the history of acts, is not in this item or this mission (decision note of 2026-11-11 on atelier-z4cr). Do not change what any command prints or refuses, and do not change the specifications in `docs/strategies/redesign-*.md`.

Expected size: about 250 lines of code moved or added and about 200 of tests, in under 15 files.

## Context

- The contract and the list of views: `docs/architecture/desk.md`. The strategy: `docs/strategies/desk.md` (Shape: "The tracker changes where the Desk needs it to"; Authority).
- This item starts only after atelier-psjw has merged, since it builds on the `Repo` that item moves into `atelier-app`. A `Repo` there holds `&Skip` and is not `Send`; whether the read-only opening is a second constructor of `Repo` or a type of its own is the implementer's choice, said in the Report.
- The switch (atelier-u1ko) moves `next/crates/` to `crates/`. Before starting, check that neither atelier-u1ko nor atelier-wd1i is in progress. If the switch merges while this branch is open, bring master in with `atelier merge --sync`.
- Sizes: complexity low, value high, risk low. The risk is a moved function that changes a command's output; the unchanged CLI tests guard it.
