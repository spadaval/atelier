---
created_at: "2026-11-11T02:17:25.617313128+00:00"
id: "atelier-1ie1"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-1ie1
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Write the Desk's API as a document first: each page's fields against the new core's views, and the views it lacks"
updated_at: "2026-11-11T05:18:58.445358053+00:00"
---

## Goal

Find out on paper whether the redesigned core can feed the Desk's pages, before any server is written. The new view layer (`next/crates/atelier-app/src/view.rs`) has three functions; the old site had views for a board, an inbox, a brief and a debrief. This item writes the Desk's API as a document and names every view the core lacks, so the next increment is sized on facts.

## Done when

- [x] `docs/architecture/desk.md` has a route table for the mission, item, inbox, repositories and live-threads pages and the act endpoint, with each response's fields.
- [x] For every field, the document names the `atelier-app` view or `atelier-records` reader in `next/` that gives it, or marks it missing. Each missing view is listed with what it must compute and a size on the three scales.
- [x] The document says what the mission page shows in place of each part of the old brief and debrief (`crates/atelier-site` and `atelier brief`, `atelier debrief` on today's binary), and which record or act each comes from.
- [x] The document lists the functions the Desk calls in `atelier-app` for the acts `ready` and `decide` and for opening a repository by path, with the result each returns, as atelier-psjw is to provide them, and marks which exist today.
- [x] The document shows, with an example read from a converted clone, how a T3 thread is tied to an item from the thread IDs the records hold, and what is shown for an item with none.
- [x] A scratch clone of this repository converted with the new converter exists outside any work tree, and the Report gives the commands that made it and `atelier check`'s result in it with the new binary.
- [x] The Report lists each page of the spike (`crates/atelier-site/deck/src/pages` on branch `spike/mission-control`) as carried over, changed, or left for a later mission, with one line of why.

## Scope

A document and a scratch clone. No crate, no app code. The Health, Insights and digest pages of the spike are out of this mission; say so in the list and do not design them.

## Context

Strategy: `docs/strategies/desk.md`. Decisions: ADR 0032. Target design of the records and commands: `docs/strategies/redesign-product.md` and `redesign-architecture.md`. The T3 Code interface is described in the 2026-11-10 note on atelier-aab1.

## Notes

- 2026-11-11 [handoff] Approved by gpt-6.1-sol (atelier-95xf) at 74b9b0d2 in round 1 with no findings. Outcome, clone commands, spike page list, missing views M1-M7 and surprises are in the Report. Not merged; the orchestrator merges.

## Report

All seven Done when lines hold. `docs/architecture/desk.md` (376 lines) has the route table for the repositories, mission, item, inbox and live-threads pages and `POST /api/act`, with each response's fields; every field names the `atelier-app` or `atelier-records` function that gives it, marked as on master, on the atelier-psjw branch, or missing (M1 to M7, each with what it must compute and its three sizes). It maps each part of the old `atelier brief` and `atelier debrief` (`Brief` and `Debrief` in `crates/atelier-app/src/mission_review.rs`) to what the mission page shows and the record or act it comes from, lists the act functions with their results and state, and shows the thread-to-item join with a real T3 thread ID read from a converted clone. Every function claim was checked by reading the code at master 3715b5f9 and at atelier-psjw 55d88b72 (`git show atelier-psjw:<path>`), not inferred from names. The document is linked from `docs/architecture/index.md`; `docs/index.md` links only that index, so it is unchanged.

### The scratch clone

Built the new binary in this worktree: `cd next && cargo build -j 4` (debug, at 3715b5f9). Then, with `TMPDIR=~/.cache/atelier-desk/tmp`:

```sh
W=/home/supadava/.t3/worktrees/atelier/atelier-1ie1; C=~/.cache/atelier-desk/clones/atelier
git clone -q --branch master /home/supadava/projects/atelier "$C" && cd "$C"
git remote set-url --push origin /nonexistent/no-push
git config user.name desk-scratch; git config user.email desk-scratch@example.invalid
pre=$(git rev-parse HEAD)                       # 009cdab9
python3 "$W/scripts/cutover/convert_records.py"
git add -A && git commit -qm "Convert Atelier records to the new layout"   # 2116defd
python3 "$W/scripts/cutover/convert_records.py" --verify "$pre"
"$W/next/target/debug/atelier" check
```

`--verify` said the tree matches a fresh conversion of 009cdab9 (191 files). `check` printed `Every record is valid: 169 items, 8 missions and config.toml.` and exited 0. Pushes from the clone fail by design. The new binary's `show atelier-z4cr` and `list --needs-human` also ran there, read-only.

For the thread example only, a second clone of that clone, `~/.cache/atelier-desk/scratch/atelier-1ie1-threads` (push URL disabled the same way), ran `atelier start atelier-1ie1` with the new binary inside this item's T3 thread, making commit ba8cd9f8. `t3_thread_read` resolved the stored `implementers` entry to this thread. It is left in place so the example can be checked; delete it when no longer wanted. The shared clone is left as a pure conversion.

### The spike's pages

- `Now.tsx` (`/`): changed. It becomes the repositories page, each repository with its counts, plus the live-thread sentence; its token totals go, since T3 Code's MCP server gives none.
- `Workspace.tsx` (`/r/:repo`, `/r/:repo/mission/:id`): carried over as the mission rail and the mission page, with its Overview, Work and Documents tabs. Its Activity tab and `/r/:repo/activity` feed are left until the history view M5 exists; `/r/:repo/loose` is changed to read `list`'s group of items in no mission; `/r/:repo/agents` becomes the live-threads page filtered to a repository; `/r/:repo/health` is left for a later mission (below).
- `Item.tsx` with `ItemDetail.tsx` (`/r/:repo/item/:id`): carried over, changed. The transitions menu is replaced by the `decide` form, the only act on an item; the old link groups become `blocked_by` and the mission; the digest view is left for later.
- `Inbox.tsx` (`/inbox`): carried over. Entries come from `list --needs-human`, draft missions and open questions instead of the old issue types.
- `Agents.tsx` (`/agents`): changed into the live-threads page. Role comes from the records' thread IDs, not from thread titles; effort, context and token columns go, since the MCP server gives none.
- `Health.tsx`: left for a later mission. Health, the feed and the database are the next mission's (strategy, Scope).
- `Insights.tsx` (`/insights`): left for a later mission, for the same reason; it needs the database.

### Missing views, in the order the mission page needs them

| ID | View | Complexity | Value | Risk |
| --- | --- | --- | --- | --- |
| M1 | Open a repository for reading from commits only, the config included | Low (~40 lines) | High | Low |
| M3 | The mission view `show <mission>` prints, with questions, moved out of `atelier-cli` | Low (~60) | High | Low |
| M2 | A mission's document paths at a commit | Low (~20) | High | Low |
| M4 | An approved item's approval time in `atelier-app` | Low (~40) | Medium | Low |
| M7 | The item view `show <item>` prints, and which record it came from | Low (~40) | Medium | Low |
| M6 | What changed in a plan between a reviewed commit and the base's tip | Low (~20) | Medium | Low |
| M5 | The history of acts (`atelier log` as specified) | High | Medium | Medium |

The act functions (`Repo::open` by path, `Skip`, `ready`, `decide`, `git::checkout_of`) all exist on the atelier-psjw branch and none on master.

### Surprises

- The reading pages need atelier-psjw too. On master, building a `Located` per item across branches (`Repo::everything`) is in `atelier-cli`; the strategy's Risks say the reading pages are not held up by atelier-psjw, which is true only once it merges.
- Even on atelier-psjw, `Repo::open` reads `config.toml` from the opened checkout's files, and `locate_at` reads the checked-out item's record from files, against the Desk's commit-only rule (M1).
- A `Repo` on atelier-psjw is not `Send` (it holds `&Skip`, which holds a `RefCell`), so the server opens it inside one blocking task.
- A converted repository holds no thread identity at all (`implementers: []`, `lead: null`, no reviews), so every converted item shows no agents until it is started anew.
- The converter moves each open mission's strategy into its directory as `design.md`, and its Sizing, Derisking and Risks are top-level headings there, which `MissionDir::section` (level 2 of `mission.md`) does not read.
- T3 Code reported `branch: null` for this thread although it works on branch `atelier-1ie1`.

### Open items

- Mapping a T3 thread that no record names to a repository (by its T3 project's workspace root) was not verified here; it is atelier-e05n's to settle.
- The token-based MCP path was not exercised by this item; the ID was resolved with this thread's own T3 tools, which call the same server.
