---
schema: atelier.item
schema_version: 3
id: atelier-613a
title: Make the first gardener run in each of the five repositories
status: done
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by:
- atelier-9dku
implementers: []
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T17:55:42.686492262Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:11.504215867Z
---

## Goal

Show the gardener's procedure works on real repositories, and put one page for each of the five in front of Sumeet: atelier, astromech, demiurge, life-on-rails and cosplay-studio. This item changes no file here. Each run is its own item, labelled `garden`, in its own repository.

## Done when

- [x] In each of the five repositories a run item labelled `garden` is approved by a reviewer from another lab and waits for its session, with Session empty and nothing merged.
- [x] Each page meets the mission criterion `one-page`: at most 60 lines, with the scorecard, the worst spots under each of the three properties or one line saying why one cannot be shown yet, and at most three proposals, each naming one root cause, its evidence with counts, and the number it should move. No page asks for an answer per trip.
- [x] Each page says which numbers mean nothing yet. Only atelier and demiurge have merged an item since the cutover of 2026-11-11, so the other three report the baseline, the checklist and what the records show, and say so.
- [x] atelier's run went first, as the procedure's trial. What it showed wrong in `garden.md` or `readiness.md` was fixed in one reviewed item before the other four started, or this item's result says nothing needed fixing.
- [x] This item is closed on the base with a result that names the five run items, their pages' line counts and each proposal's title.

## Scope

In: one run per repository by `skills/agent-factory/procedures/garden.md`, steps 1 to 7 up to the approval; and one item in atelier for what the trial shows wrong in the procedure.

Out:

- The session and everything it closes: settling a proposal, filing an agreed one as an item, the run's merge, closing atelier-j86z. They are Sumeet's, with the thread he talks to.
- Adding or removing a rule, a check, a merge check or a hook in any repository. The mission's Authority forbids it from a run; such a change is a proposal on the page.
- A snapshot, a run of `scripts/scorecard/`, and any measure a run carries. The comparison is atelier-pwhn.
- astromech-6ec9, lor-pmzz and lor-sghw, their branches, and every worktree a run did not make. They are Sumeet's parked work.
- A second run anywhere, and a scheduler (atelier-jy8k).

## Design

- **Who runs what.** The mission's lead starts one gardener thread per repository, Claude Opus 5.5, each told to start its own reviewer, GPT-6.1 Sol, as a T3 thread. The lead writes no page and reviews none.
- **atelier first, then four at once.** The four product runs start when atelier's page is approved and the procedure is fixed if it needed it. Each gardener runs one build or test command at a time.
- **Who said go.** Each run item is made ready `--by "sumeet, through mission atelier-295u"`. The mission is his go-ahead for these five runs and no others.
- **Every run here is a first run.** No repository has a page yet. atelier-j86z carries the `garden` label and is the old procedure's digest, not a run: it has no window. atelier's run reads its ten entries as evidence, says on its page which the rewrite made moot, and may propose that the session close it. It does not close it.
- **Where a run works.** Planning (`new`, `set`, `ready`) happens in the checkout that has the base: `/home/supadava/.t3/worktrees/atelier/t3-f937cc1a` for atelier, `/home/supadava/.t3/worktrees/astromech/t3-e6ec80d6` for astromech, and `~/projects/demiurge`, `~/projects/life-on-rails` and `~/projects/cosplay-studio`. Other threads and Sumeet share those: a run stages by explicit path and commits its record at once. The run's branch lives in a worktree the gardener makes for it.
- **Pushing.** atelier's `submit` pushes the run's branch and opens its pull request. The other four configure no forge: the gardener pushes the base after planning, as each `AGENTS.md` says, and pushes nothing else. A push refused for credentials is reported, not worked around.
- **What a page may hold.** astromech and demiurge are public, and a page ends up in Git. A page and its artifacts hold numbers, item IDs, commit hashes, paths inside the repository and the gardener's own words. They hold no text quoted from a transcript, no thread title, no path under a home directory and no personal detail. Nothing from a repository or a transcript goes to an outside service.
- **Baseline.** The November baseline's numbers for all five are in atelier, `docs/architecture/quality/scorecard-baseline-2026-11.md`.

## Report

The first gardener run in each of the five repositories is approved and waits for its session: atelier-0019, astromech-qjle, worldcrafter-gq2k, lor-j26w and cosplay-studio-ktgi. Pages are 36 to 46 lines with nine proposals in all.

### The five runs

Each is approved by a GPT-6.1 Sol reviewer, with Session empty and nothing merged. Each gardener was a Claude Opus 5.5 thread. The page is the run item's Report on its branch; lines are counted from `Window:` to the end of Fixed and Settled.

| Repository | Run item | Worktree, on the item's branch | Page lines | Review rounds | Proposals |
| --- | --- | --- | --- | --- | --- |
| atelier | atelier-0019 (pull request 148) | `~/.t3/worktrees/atelier/garden-atelier-0019` | 46 | 2 | P1 One reader for record Markdown. P2 Lint every Rust merge with clippy, and drop the merge checks CI repeats. P3 Give every bug item a cause, or drop it. |
| astromech | astromech-qjle | `~/.t3/worktrees/astromech/garden-astromech-qjle` | 36 | 2 | P1 Run CI before a merge again. |
| demiurge | worldcrafter-gq2k | `~/.t3/worktrees/demiurge/garden-worldcrafter-gq2k` | 40 | 2 | P1 Run `bun run check` in `app/` on every merge. P2 Give the two bug items a cause, or drop them. |
| life-on-rails | lor-j26w | `~/.t3/worktrees/life-on-rails/garden-lor-j26w` | 36 | 1 | P1 Give each of the seven bug items a cause, or drop it. |
| cosplay-studio | cosplay-studio-ktgi | `~/.t3/worktrees/cosplay-studio/garden-cosplay-studio-ktgi` | 40 | 2 | P1 Groom the 27 converted July items. P2 Run `bun run check` and `bun run test` at every merge. |

Each page holds the scorecard, a line saying which numbers mean nothing yet, the checklist's result, the worst spots under the three properties, and its proposals, each with one cause, counted evidence and the number it should move. No page asks for an answer per trip. Only atelier (7 items) and demiurge (1 item) had merged anything in their windows, which all start at the cutover on 2026-11-11 at about 15:20 UTC; the other three report the baseline, the checklist and what the records show.

Checked by the lead on each branch: status `approved`, a clean worktree, an empty Session, at most three proposals, and no path under a home directory in the item or its artifacts.

### What the checklist showed

Lines met of nine, on each base: atelier 4, astromech 3 (one could not be tested), demiurge 0, life-on-rails 1 (two could not be tested), cosplay-studio 3. Three lines are not met in all five: no merge runs every documented check (line 7), no verify skill or one whose launch cannot be shown (line 8), and bug items with no `caused_by` (line 9; 39, 6, 2, 7 and 2 items). Four proposals of the nine are about line 7 and three about line 9.

### The trial, and what it changed

atelier's run went first. Its gardener reported 19 places where `garden.md` or `readiness.md` was wrong, unclear or made it guess. atelier-f6dv (pull request 149) fixed the ones a sentence would settle before the other four started; the worst was that step 1 did not say to start the run in its own worktree, and `atelier start` in the base's checkout switches that checkout.

What the four later runs each still had to guess is in atelier-h201. That `atelier status` offers `atelier merge` on an approved run is a question for Sumeet, atelier-e7iy. What the cutover left stale in each repository's own docs, skills and scripts is atelier-52g7.

### What the gardeners did on their branches

Each change waits for its run's merge: atelier, four doc lines naming deleted paths, and two deferred findings settled (one dropped as fixed, one taken into atelier-qf7u); astromech, `CONTRIBUTING.md` and two dead links, plus the bug astromech-xj3u filed on `main`; demiurge, `AGENTS.md` naming `app/`'s commands and five files formatted so the root lint passes; life-on-rails, a `CLAUDE.md` that imports `AGENTS.md`, four corrections to `AGENTS.md` and the verify skill's removed commands; cosplay-studio, the same `CLAUDE.md`, the commands in `AGENTS.md`, and three docs naming removed commands.

### Left for the session

For each run: settle its proposals with Sumeet, write the outcome under Session, file what he agrees as items on that repository's base, and run `atelier merge` from the run's worktree, taking the run over with `atelier start <id>` there. In the four product repositories, push the base afterwards, as each `AGENTS.md` says. atelier's page also says which entries of atelier-j86z the rewrite made moot, so the session can close it.

## Context

The procedure: `skills/agent-factory/procedures/garden.md` and `readiness.md` (pull requests 146 and 144). The log: pull request 145. The checklist's trial already ran in atelier and demiurge; its results are in atelier-vl5n's Report.
