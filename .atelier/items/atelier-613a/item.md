---
schema: atelier.item
schema_version: 3
id: atelier-613a
title: Make the first gardener run in each of the five repositories
status: draft
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by:
- atelier-9dku
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:11.504215867Z
---

## Goal

Show the gardener's procedure works on real repositories, and put one page for each of the five in front of Sumeet: atelier, astromech, demiurge, life-on-rails and cosplay-studio. This item changes no file here. Each run is its own item, labelled `garden`, in its own repository.

## Done when

- [ ] In each of the five repositories a run item labelled `garden` is approved by a reviewer from another lab and waits for its session, with Session empty and nothing merged.
- [ ] Each page meets the mission criterion `one-page`: at most 60 lines, with the scorecard, the worst spots under each of the three properties or one line saying why one cannot be shown yet, and at most three proposals, each naming one root cause, its evidence with counts, and the number it should move. No page asks for an answer per trip.
- [ ] Each page says which numbers mean nothing yet. Only atelier and demiurge have merged an item since the cutover of 2026-11-11, so the other three report the baseline, the checklist and what the records show, and say so.
- [ ] atelier's run went first, as the procedure's trial. What it showed wrong in `garden.md` or `readiness.md` was fixed in one reviewed item before the other four started, or this item's result says nothing needed fixing.
- [ ] This item is closed on the base with a result that names the five run items, their pages' line counts and each proposal's title.

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

## Context

The procedure: `skills/agent-factory/procedures/garden.md` and `readiness.md` (pull requests 146 and 144). The log: pull request 145. The checklist's trial already ran in atelier and demiurge; its results are in atelier-vl5n's Report.
