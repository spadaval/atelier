---
created_at: "2026-11-08T05:35:32.276590166+00:00"
id: "atelier-l58v"
issue_type: "task"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-stnv"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Let Agent Factory say no: merge verdicts, won't-fix, stop signals, a gardener and a verification standard"
updated_at: "2026-11-08T05:35:32.276590166+00:00"
---

## Goal

Change the agent-factory skill (spadaval/.agents, `skills/agent-factory`) so that reviewers, orchestrators, workers and planners can say no and make it stick, add a gardener role that keeps a codebase healthy on a daily cadence, and set a standard for each repository's verify skill. Prove with evals built from the audit's real cases that the change helps where review hurt without losing what review caught.

## Done when

- [ ] review.md: the reviewer opens with a verdict of merge, shrink or reject, then complexity, value and risk each sized in three levels with written anchors and a one-line justification, then line-level findings. A reject names the outcome the change serves and why its cost outweighs that. Shrink and reject are each recorded as a `fix_now` finding whose text opens with the verdict, so merge's existing gates block them; merge's gates stay as they are. A shrink is settled by the fix and a fresh approval. A reject that stands ends the work with `decline`. The accountable agent, never the author, may override either with a `no_action` disposition giving its reason. Reviewers still report every finding they can support.
- [ ] Findings end as fixed, deferred, won't-fix (real, not worth fixing, with a reason) or no action (the finding is wrong). A won't-fix on a high-severity finding or on anything touching the strategy's criteria goes to the human: agents leave the finding undisposed and label the item `needs-human`, and the human records the disposition. The skill says who records each other decision.
- [ ] orchestrate.md: the reviewer's recommendation is the default; fixing a deferred finding now is a scope decision with a recorded reason. Stop signals (a fix past about twice its estimate, a fix drawing findings on its own changes, a third review round) mean choosing between merge as is, shrink, or discard, where discard means `decline` with what it taught recorded. The estimate is the size (lines or files changed) the orchestrator or worker writes in the assignment or a decision note when the work starts. One rule overrides the reviewer's recommendation: a new finding on the fix of a low-severity finding is deferred, unless it shows a regression that fix introduced. Items that depend on an open human decision are not planned in detail or reviewed until it is decided. Deferring uses the one-step command.
- [ ] implement.md: a worker past about twice its estimate stops and reports instead of pushing through, and anyone may propose not doing work.
- [ ] plan.md: shared code and APIs are planned docs first (the README or tutorial before the code), and a planner may propose declining an item.
- [ ] A new `procedures/garden.md` defines the gardener: construction review across recent merges; finding mistake classes that have happened twice and fixing each at the highest level that works (architecture, then a check, then a doc), and deleting rules whose mistake can no longer happen; sweeping deferred findings and backlog, proposing declines; keeping the repository's verify skill and feature map current. It reviews what merged since its last run (the last 7 days on a first run) and counts mistake classes over the last 30 days. A run covers one repository and opens at most one pull request plus filed issues, reviewed like any change; it never touches a branch in flight. SKILL.md lists it.
- [ ] A new reference sets the verify-skill standard: a control CLI an agent drives the app with, a feature map of what the app does and what proves each feature works, Launch, Doctor, Drive, Evidence and Cleanup sections, and one end-to-end run before anyone relies on it. implement.md has authors use the repository's verify skill, where one exists, before review.
- [ ] references/atelier.md covers won't-fix, declined and the one-step defer as the binary's help describes them (installed from the sibling item before evals run).
- [ ] The agent-facing skill stays within atelier-qj9d's limits, counted by its rule (a role's read path is SKILL.md, its procedure and every file the procedure links, counted whole, plus the Atelier reference for roles that run `atelier`): at most 1,000 lines, SKILL.md at most 100, each role's read path at most 300 and the orchestrator's 450. The Report gives the counts before and after; an overrun needs a reason there.
- [ ] New eval scenarios in `evals/agent-factory/scenarios/` built from the audit, each with setup, prompt and grader: a rare-edge chain modelled on astromech-of26 (a low-severity finding the reviewer called rare and deferred; pass when it is won't-fix or deferred, not fixed); a costly hardening diff modelled on atelier-hwxm (a large fix for a low-severity finding on a path planned for deletion; pass when review returns shrink or reject); and two controls, a real cross-repository regression modelled on atelier-2ygf and a plan defect modelled on lor-0rnt (pass when each is found and recommended fix now).
- [ ] All eight scenarios (the four new and the four existing) run at the current skill (main) and at the new one, 3 runs each with Opus on the skill arm, with the same installed atelier binary built with atelier-stnv. Every run is read by hand. Results are recorded in `evals/agent-factory/results/`. The new skill passes more runs than main on the rare-edge chain and the costly hardening scenarios, and no fewer on every other scenario. If main already passes one of those two, it shows nothing: tighten that scenario's pressure until main fails, and rerun both refs.

## Scope

`skills/agent-factory` and `evals/agent-factory` in spadaval/.agents, edited in a worktree (`git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/<id> -b <id> main`; the live checkout never changes branch), and this item's records. `constitution.md` and `maintainers/` change only to fix links or to add the new scenarios to the pressure table. Not Atelier's code.

## Context

Agreed with Sumeet in conversation on 2026-11-08 after an audit of review records in atelier, astromech and life-on-rails (about 175 findings sampled). Review found real defects about three times as often as it did harm, and no sampled finding was wrong. The harm came after review: orchestrators fixed findings the reviewer marked defer (atelier: 122 of about 242 deferred findings were fixed anyway, 80 with no reason beyond "Fixed in <sha>"), several "easy" fixes grew to hundreds of lines (astromech-u6ak, astromech-emsi, the astromech clock-skew chain, atelier-hwxm), and nothing in the process lets anyone decide not to do real work. A defer costs a follow-up issue per finding, and backlog is the only place a no can go (atelier has 100 backlog items, astromech 71 open follow-ups from dispositions).
