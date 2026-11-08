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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Let Agent Factory say no: merge verdicts, won't-fix, stop signals, a gardener and a verification standard"
updated_at: "2026-11-08T05:35:32.276590166+00:00"
---

## Goal

Change the agent-factory skill (spadaval/.agents, `skills/agent-factory`) so that reviewers, orchestrators, workers and planners can say no and make it stick, add a gardener role that keeps a codebase healthy on a daily cadence, and set a standard for each repository's verify skill. Prove with evals built from the audit's real cases that the change helps where review hurt without losing what review caught.

## Done when

- [ ] review.md: the reviewer opens with a verdict of merge, shrink or reject, then complexity, value and risk each sized in three levels with written anchors and a one-line justification, then line-level findings. A reject names the outcome the change serves and why its cost outweighs that. Shrink and reject are recorded so they block merge until the accountable agent (never the author) decides; overriding one needs a recorded reason; a reject that stands ends the work (`decline`). Reviewers still report every finding they can support.
- [ ] Findings end as fixed, deferred, won't-fix (real, not worth fixing, with a reason) or no action (the finding is wrong). A won't-fix on a high-severity finding or on anything touching the strategy's criteria goes to the human.
- [ ] orchestrate.md: the reviewer's recommendation is the default; fixing a deferred finding now is a scope decision with a recorded reason. Stop signals (a fix past about twice its estimate, a fix drawing findings on its own changes, a third review round) mean choosing between merge as is, shrink, or discard with what it taught recorded. A new finding on the fix of a low-severity finding is deferred by default. Items that depend on an open human decision are not planned in detail or reviewed until it is decided. Deferring uses the one-step command.
- [ ] implement.md: a worker past about twice its estimate stops and reports instead of pushing through, and anyone may propose not doing work.
- [ ] plan.md: shared code and APIs are planned docs first (the README or tutorial before the code), and a planner may propose declining an item.
- [ ] A new `procedures/garden.md` defines the gardener: construction review across recent merges; finding mistake classes that have happened twice and fixing each at the highest level that works (architecture, then a check, then a doc), and deleting rules whose mistake can no longer happen; sweeping deferred findings and backlog, proposing declines; keeping the repository's verify skill and feature map current. At most one pull request per run plus filed issues, reviewed like any change; never touches a branch in flight. SKILL.md lists it.
- [ ] A new reference sets the verify-skill standard: a control CLI an agent drives the app with, a feature map of what the app does and what proves each feature works, Launch, Doctor, Drive, Evidence and Cleanup sections, and one end-to-end run before anyone relies on it. implement.md has authors use the repository's verify skill, where one exists, before review.
- [ ] references/atelier.md covers won't-fix, declined and the one-step defer as the binary's help describes them (installed from the sibling item before evals run).
- [ ] The agent-facing skill stays within atelier-qj9d's limits (at most 1,000 lines; each role's read path at most 300, the orchestrator's 450), or the Report gives the counts and why an overrun is worth it.
- [ ] New eval scenarios in `evals/agent-factory/scenarios/` built from the audit, each with setup, prompt and grader: a rare-edge chain modelled on astromech-of26 (a low-severity finding the reviewer called rare and deferred; pass when it is won't-fix or deferred, not fixed); a costly hardening diff modelled on atelier-hwxm (a large fix for a low-severity finding on a path planned for deletion; pass when review returns shrink or reject); and two controls, a real cross-repository regression modelled on atelier-2ygf and a plan defect modelled on lor-0rnt (pass when each is found and recommended fix now).
- [ ] The new scenarios run at the current skill (main) and the new one, 3 runs each with Opus on the skill arm, with the same installed atelier binary; the four existing scenarios run 3 times each at the new skill. Every run is read by hand. Results are recorded in `evals/agent-factory/results/`. The new skill passes more runs than main on the first two scenarios and no fewer on every other scenario. A scenario main already passes is recorded as no evidence, per the method.

## Scope

`skills/agent-factory` and `evals/agent-factory` in spadaval/.agents, edited in a worktree (`git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/<id> -b <id> main`; the live checkout never changes branch), and this item's records. `constitution.md` and `maintainers/` change only to fix links or to add the new scenarios to the pressure table. Not Atelier's code.

## Context

Agreed with Sumeet in conversation on 2026-11-08 after an audit of review records in atelier, astromech and life-on-rails (about 175 findings sampled). Review found real defects about three times as often as it did harm, and no sampled finding was wrong. The harm came after review: orchestrators fixed findings the reviewer marked defer (atelier: 122 of about 242 deferred findings were fixed anyway, 80 with no reason beyond "Fixed in <sha>"), several "easy" fixes grew to hundreds of lines (astromech-u6ak, astromech-emsi, the astromech clock-skew chain, atelier-hwxm), and nothing in the process lets anyone decide not to do real work. A defer costs a follow-up issue per finding, and backlog is the only place a no can go (atelier has 100 backlog items, astromech 71 open follow-ups from dispositions).
