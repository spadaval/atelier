---
created_at: "2026-11-07T17:18:36.057434340+00:00"
id: "atelier-qj9d"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-qj9d
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Cut the agent-factory skill to what a (sub)agent needs to do its role"
updated_at: "2026-11-07T18:28:59.753623565+00:00"
---

## Goal

The agent-factory skill (spadaval/.agents, `skills/agent-factory`) is about 2,220 lines an agent can be sent to read, not counting the constitution (181), `maintainers/` (267) or the evals. Much of it says the same thing in several places, restates what Atelier's CLI already enforces and lists, or patches edge cases; the simpler-lifecycle mission (atelier-mmpv) made one file longer (references/atelier.md 198 to 234 lines). Sumeet's goal, 2026-11-07: minimise the skill as far as possible while keeping the basic instructions a (sub)agent needs to carry out its role.

## Done when

- [x] The agent-facing skill (everything under `skills/agent-factory` except `constitution.md` and `maintainers/`) is at most 1,000 lines, and SKILL.md at most 100, counted at the current ~80-column wrap.
- [x] Each role's read path is at most 300 lines, and the orchestrator's at most 450. A role is each subskill that remains; its read path is SKILL.md, its procedure, and every file the procedure links (counted whole, conditional links included), plus the Atelier reference for every role that runs `atelier` commands. A table in the Report gives each role's path and count at 4d88ffb (recounted by this rule) and after.
- [x] Nothing on an agent's path links to or tells it to read the constitution, which is a manifesto about the skill, not part of it. The one rule agents need from it (strategy changes only by human direction or authority written into the strategy) is stated once, in SKILL.md's core rules, and the four links to it from plan, orchestrate, decide and strategic-plans go.
- [x] Each rule has one home: no rule is stated in two files, and nothing restates what `atelier --help`, `atelier <cmd> --help` or a refusal message already tells the agent (record sections, flags, transition names); the skill points to the help instead. Where the skill now relies on the help, the Report names the help text it relies on, confirmed to exist.
- [x] The four eval scenarios (`evals/agent-factory/scenarios`: refused-land-push, vacuous-test-review, done-when-rewrite, trivial-typo) run on the skill arm, 3 runs each with Opus and with Sonnet, at the old skill (main 4d88ffb) and the new one, with the same installed `atelier` binary, every run read by hand: the new skill passes at least as many runs per scenario and model as the old. If a cell drops, read the failing run: if removed guidance caused it, restore that guidance; if not, rerun that cell at 6 runs per arm and compare again. Results are recorded in `evals/agent-factory/results/`.
- [x] The mission-shaping micro-test (`evals/agent-factory/results/2026-11-04-mission-shaping.md`, its input file and prompt) is rerun old skill against new, 2 runs each with Opus and Sonnet, read by hand: the new skill keeps the behaviours that test showed (a transcript measurement is not made a mission criterion; criteria do not restate single items). Recorded alongside.
- [x] A fresh agent given only the new skill, astromech's AGENTS.md and the CLI help answers, with no contradiction found: the lifecycle order, what makes an approval stale and when to file a loose end (atelier-mmpv's legible criterion); and, as an orchestrator, who arranges review, what the reviewer is given and not given, who decides findings on an author's own work, and who records the approval.

## Scope

`skills/agent-factory` in spadaval/.agents (edited in a worktree, `git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/<id> -b <id> main`; the live checkout never changes branch), its evals results, and this item's records. Subskills may be merged, shortened or removed if no role needs them; say which and why. `constitution.md` and `maintainers/` are not edited except to fix links. Not Atelier's code or docs. Do not reinstall `atelier` between the old and new eval runs.

## Context

The eval harness: `evals/agent-factory/README.md`; method: `skills/agent-factory/maintainers/skill-evaluation.md` (baseline first, read every run by hand). The old-skill runs of the four scenarios are being taken by the orchestrator now, into `~/.cache/af-evals/runs/qj9d-baseline/{opus,sonnet}`. Current file sizes: tracker-records.md 276, atelier.md 234, SKILL.md 191, orchestrate.md 185, plan.md 162, strategic-plans.md 161, branches-and-worktrees.md 121, finding-disposition.md 105, repository-shape.md 89, implement.md 85.

## Notes

- 2026-11-07 [observation] Final fresh read (seventh round) of skill e2671ec, by a fresh subagent given only the skill's SKILL.md, procedures/ and references/atelier.md, astromech origin/main:AGENTS.md, and the help of atelier 0.3.1+e03d6b71 (atelier --help; start; review, review request, finding, approve, disposition, trivial; merge; land; issue create, link, transition; evidence record). Its answer, condensed:

  1. Lifecycle:
     - `atelier start <id>`; implement, with code and records committed together; tick every Done when line and write the `## Report`.
     - `atelier review request <id>` refuses until both hold. The reviewer records findings, and the author fixes each standing `fix_now` finding in a later commit, which a reviewer reviews. Deferred or dismissed findings get a disposition. The reviewer approves.
     - `atelier merge <id>` makes one merge commit and marks the work, and every open child, done. There is no close step.
     - Run `land` before every handoff; push only if it exits zero. A change with no issue takes the light path.
  2. Stale:
     - Only a code change since the approved commit makes it stale. Code is anything outside .atelier/issues, reviews and evidence, so policy, config and strategy files count.
     - Not stale: record-only commits, a clean base merge, a Done when line rewritten after start (shown, refuses nothing).
     - Independent Review still has rewritten requirements, fixes and non-trivial conflict resolutions reviewed again. No rebase after review.
  3. Loose end:
     - What it is: work that changes no code and can only follow a merge or release; an independent validation is a validation item instead.
     - When: file it when you plan the item or as soon as such work turns up. It goes on the base as a reviewed planning change, even while its issue is in flight: `issue create --issue-type loose_end`, then `issue link ... --role blocked_by`.
     - Its line: the plan owner rewrites or drops the replaced Done when line on the work branch.
     - A release tag is filed on the release branch. Pre-merge parts stay Done when lines.
     - Done on the base after the merge with no branch, review or merge; it never holds a mission open.
  4. The orchestrator dispatches reviewers, for its own changes too; a lone author spawns its own. An implementer under an orchestrator reports review outstanding. With no reviewer, keep the change unmerged and ask the user.
  5. Reviewer:
     - Given: the review-request brief (Goal, Done when, edits after start, commits, earlier findings and dispositions, recorded proof, policy rules, recording commands), plus the proof that ran as commands and results, constraints the brief lacks such as Scope, both sides of conflicting requirements, and review.md's output format. Fresh context, no branch or worktree.
     - Not given: the author's rationale, self-assessment, notes or Report (any met in the diff are weighed as claims), the orchestrator's conversation, or any pre-judging.
  6. Findings:
     - The accountable agent decides each with review.md's Recommendations, but an author never decides findings on its own work.
     - An author that is also the accountable agent fixes every recommended FIX NOW or sends a rebuttal (a fact or a test) to a reviewer, who decides and records it. A reviewer gives deferred or dismissed findings their disposition and files the follow-up.
     - An open FIX NOW blocks the merge.
  7. Approval: the reviewer records it under its own identity, with the command the brief prints; an agent reviewer records it itself. A human's plan approval is recorded under the human's name.
  8. "No outright contradictions, and every cross-link anchor resolves." Two-way readings it listed, none a contradiction:
     - (a) astromech AGENTS.md has no trivial-base exception, while atelier.md allows one "where policy allows";
     - (b) AGENTS.md's unconditional "no catch-up merge" against atelier.md's conditions for merging the base;
     - (c) AGENTS.md's "Don't do" for decision records against agents recording decisions they may make;
     - (d) the help's light-path "approval of its head";
     - (e) starting a child before its parent has a branch;
     - (f) whether a lone author files its own loose end;
     - (g) which branch a reviewer commits a deferral's follow-up on;
     - (h) AGENTS.md calls marking a loose end done planning, while atelier.md says the result needs no review;
     - (i) AGENTS.md never states CI as a merge condition.
     It also found that the Words section defines all five words AGENTS.md says it does.

## Report

The agent-facing skill is cut from 2,225 lines to 817 (SKILL.md 191 to 96), and every role's read path is within its limit: the orchestrator's is 419, and the largest other is plan's at 295. Skill: branch `atelier-qj9d` in spadaval/.agents, head **82ca7e4** (8586d2b, e22cc1a, 4b17ecc, 75d405c, 2eb0f93, 2c073e3, 3af0fb7, 48caade, e2671ec, 82ca7e4 on main 4d88ffb). It is committed only, not pushed or merged. 82ca7e4 fixes the review's `fix_now` finding atelier-2xlu: plan.md's Ready Work makes the human's plan approval a ready condition again, unless the strategy's Authority section delegates it. It also has review.md's DEFER file its follow-up with `--discovered-from`, and adds the eval results file. No agent-facing file links to the constitution or tells an agent to read it (`grep -ri constitution SKILL.md procedures references agents` finds nothing). The strategy-authority rule is stated once, in SKILL.md's Respect authority core rule. The fresh-agent read ran seven rounds; the last, on e2671ec, answered all seven questions and found no contradiction or stale pointer (observation note of 2026-11-07).

Evals (lines 5 and 6) are recorded in `evals/agent-factory/results/2026-11-07-skill-cut.md` on the skill branch. The orchestrator ran them on old 4d88ffb against new e2671ec, with the same `atelier 0.3.1+e03d6b71`, and read every run by hand. By hand count the new skill passes at least as many runs as the old in every scenario and model:

| Scenario | Opus, old → new | Sonnet, old → new |
| --- | --- | --- |
| refused-land-push | 3/3 → 3/3 | 3/3 → 3/3 |
| vacuous-test-review | 3/3 → 3/3 | 3/3 → 3/3 (one doubtful pass) |
| done-when-rewrite | 3/3 → 3/3 | 3/3 → 3/3 |
| trivial-typo | 2/3 → 2/3 | 0/3 → 2/3 |

The results file traces each new-skill failure to a cause that predates the cut. The mission-shaping micro-test, at 2 runs per arm on each model, kept both behaviours in all 8 runs: the transcript measurement stayed out of mission criteria, and no criterion restated a single item.

Counts come from a script that applies the Done when rule. A role's path is SKILL.md, its procedure, and every file the procedure links, counted whole. The Atelier reference is counted for every role, since each may record to the tracker or merge. **Raw** is `wc -l`; **80-col** counts each line as ceil(length/80). The only line over 80 columns is SKILL.md's front-matter description (393 characters), unchanged from 4d88ffb.

| | 4d88ffb read path | raw / 80-col | 82ca7e4 read path | raw / 80-col |
| --- | --- | --- | --- | --- |
| Agent-facing total | 23 files | 2225 / 2307 | 13 files | 817 / 822 |
| SKILL.md | | 191 / 229 | | 96 / 100 |
| orchestrate | SKILL, orchestrate, review, atelier, constitution, branches-and-worktrees, finding-disposition, implementation-candidates, submodel-selection | 1208 / 1266 | SKILL, orchestrate, review, atelier | 419 / 423 |
| plan | SKILL, plan, atelier, constitution, strategic-plans, tracker-records | 1205 / 1262 | SKILL, plan, atelier | 295 / 299 |
| implement | SKILL, implement, atelier, branches-and-worktrees, finding-disposition, good-code, implementation-candidates, tracker-records | 1132 / 1178 | SKILL, implement, atelier | 263 / 267 |
| install | SKILL, install, atelier, branches-and-worktrees, repository-shape, strategic-plans, tracker-records | 1139 / 1198 | SKILL, install, atelier | 258 / 262 |
| readiness | SKILL, readiness, atelier, repository-shape | 544 / 589 | merged into install | |
| review | SKILL, review, atelier, finding-disposition, good-code | 649 / 696 | SKILL, review, atelier | 291 / 295 |
| validate | SKILL, validate, atelier, finding-disposition, tracker-records | 856 / 904 | SKILL, validate, atelier | 239 / 243 |
| decide | SKILL, decide, atelier, constitution, tracker-records | 931 / 976 | SKILL, decide, atelier | 233 / 237 |
| docs | SKILL, docs, atelier, tracker-records | 735 / 779 | merged into implement | |
| audit | SKILL, audit, atelier, tracker-records | 736 / 780 | SKILL, audit, atelier | 226 / 230 |
| migrate | SKILL, migrate, atelier, branches-and-worktrees | 619 / 663 | SKILL, migrate, atelier | 245 / 249 |
| diagnose | SKILL, diagnose, atelier | 466 / 508 | SKILL, diagnose, atelier | 235 / 239 |

The skill now points to help text instead of restating it. Each piece below was confirmed against the installed `atelier 0.3.1+e03d6b71`, by grepping the command's output for the quoted text:

- `atelier --help`:
  - "One issue, in order: start it; implement; tick every Done when line and write its ## Report; review request, which refuses until both hold; fix each fix_now finding and get the fix reviewed again; approve; merge".
  - "Only a code change makes an approval stale; a Done when line rewritten after start is shown in the review brief and by merge --check, and refuses nothing."
  - "... is a loose end (--issue-type loose_end); an independent validation is a validation item instead."
  - "A change with no issue takes review request and merge with no ID on its own branch."
  - "Run land before every handoff."
- `atelier issue create --help`: `--body` lists each type's `## ` sections, and `--issue-type` is there.
- `atelier issue link --help`: "Readiness: blocked_by (the source waits for the target)".
- `atelier issue transition --help`: "omit to inspect options and blockers".
- `atelier review request --help`: "It never includes the author's notes or Report, and it refuses while ... an unticked Done when line or an empty Report."
- `atelier review approve --help`: `--branch` covers "a release, docs or a mockup on the light path".
- `atelier review finding --help`: "an approval after its fix resolves a fix_now finding, and defer or no_action needs a disposition before merge".
- `atelier review disposition --help`: "A finding deferred or dismissed needs one before merge; a fix_now finding needs none".
- `atelier review trivial --help`: "With --base and no owner, it covers exactly one commit already on the base branch."
- `atelier merge --help`: `--sync` "merge the base into the work branch and report whether the approval survived".
- `atelier land --help`: `--push` pushes "when its gates pass", and "a refusal ... pushes nothing, and it or a failed push exits non-zero".
- `atelier evidence record --help`: "--criterion ... A mission success criterion this receipt judges".
- `atelier issue note --help`: the `--kind` list includes decision and handoff.
- `atelier start --help`: "its recorded branch, else on the base a new branch named with its ID, else the current branch".
- Refusals and printed steps, checked on a throwaway fixture:
  - `review request` refuses with "1 Done when line unticked ...; Report empty" plus a Next line.
  - `merge --check` lists each failed gate with its Next step.
  - `atelier init` prints its next steps, including "Commit the install on main; it predates the gates, so it needs no review".
  - The `merge_base_records` refusal tells the agent to merge the base (crates/atelier-cli/src/commands/merge.rs:332).
- Two fixture checks behind sentences the help leaves open:
  - `atelier start <child>` on the base lands on the parent's branch.
  - `atelier review approve <mission>` records on the base.

### Decisions

- **Subskills merged:**
  - `docs` into `implement`. Docs work is implementing a docs change; its four rules are now implement's Docs bullet, and its handoff fields are in implement's Handoff.
  - `readiness`, with `references/repository-shape.md`, into `install`. The readiness test is the same list install makes the entry point answer.
  - All other subskills remain, each shortened.
- **References removed:** all but `atelier.md`. Each surviving rule went to the one file whose role needs it:
  - `tracker-records`: its templates restated `issue create --help`. The Report went to implement's Handoff; what makes an issue, mission or increment ready to plan, to plan; the decision-record test and decision notes, to atelier's Records; loose ends and validation items, to atelier; failure-cause vocabulary, to validate; the handoff note, to SKILL's Handoff.
  - `strategic-plans`: location, size, sections and how to change it went to plan's Strategy File and Strategic Revision. The worked example was dropped.
  - `finding-disposition`: who decides went to SKILL's Findings. FIX NOW, DEFER, NO ACTION and the complexity test went to review's Recommendations, which orchestrate links.
  - `good-code`: one owner, boundaries and no speculative machinery went to SKILL's simplest-thing core rule; the integration lens, to review's second question.
  - `implementation-candidates`: candidate fields and dispositions went to orchestrate; the candidate handoff, to implement.
  - `submodel-selection`: one paragraph of orchestrate's Assignments.
  - `branches-and-worktrees`: preserving others' changes, the baseline and keeping branches few went to SKILL's core rules. Workspace, one branch per PR and Atelier's no-stacking rule went to orchestrate's Assignments; cleanup and archive tags, to orchestrate's Closing a Mission.
- **Constitution:** removed the four links (plan twice, orchestrate, decide) and the one in strategic-plans, and SKILL's "reason from the Constitution" paragraph. SKILL.md no longer points to `maintainers/skill-evaluation.md`, the only remaining route from the skill to the constitution; the eval harness strips `maintainers/` anyway.
- **Atelier reference:** now 104 lines, from 234. It keeps:
  - who takes each of `atelier --help`'s seven steps, which the help doesn't say;
  - what counts as code, and when to merge the base in;
  - planning on the base, including that a started item's record changes only on its branch;
  - the decision-record test;
  - when and where to file a loose end, and when a validation item is needed;
  - a compact Words section, kept because astromech's AGENTS.md points to it for check, review, land, done and close.
- **Cut as restated help, edge case or rationale:**
  - the record templates, the flag and transition lists, `--commit-records` and `--stage-records`;
  - correcting review records, `bundle preview`, `doctor` and `prune`;
  - non-Atelier branch naming and stack depth, and `$generate-html-plan`;
  - the strategy example, and the constitution's why.
- **Form:** each rule is a sentence with its reason. The remaining "never"s carry theirs: plain `git merge` runs no gate, and rebasing orphans the commits that reviews pin.

### Deviations

- **The Atelier reference is counted on every role's path.** The Done when line counts it only for roles that run `atelier` commands; counting it everywhere is conservative, and it raises the 4d88ffb figures for diagnose and migrate as well. Both raw and 80-column counts are given. Plan's path is exactly 299 in 80-column lines.
- **Seven fresh reads, not one.** Each round raised new two-way readings, each less weighty than the last. Fixing them:
  - aligned the steps with the help's seven, so approval follows the fix;
  - always re-reviews a finding's fix;
  - said who records a human's approval, how a planning review is briefed and recorded, who rewrites a line a loose end replaces, and where a release tag's loose end goes;
  - let small work take the light path and keep its review;
  - counted a merge gate as policy that forbids an unreviewed merge;
  - limited review coverage to planning records, not Reports or notes;
  - added that the review brief needs the constraints it lacks, such as Scope;
  - kept a lone author from overturning its own findings;
  - allowed a deciding reviewer to file a deferral's follow-up.

### Open items

- **Re-review:** the fix in 82ca7e4 for finding atelier-2xlu needs re-review before the skill branch merges into spadaval/.agents main.
- **Harness leak, suspected:** the eval reader suspects the harness may leak the user's global CLAUDE.md into runs. One new Sonnet vacuous-test-review run said "since you review after merge", which appears in no scenario or skill text. It is unverified, and it would reach both arms.
- **Grader bugs:**
  - done-when-rewrite requires an `atelier evidence` receipt as "fresh proof", which caused four false failures;
  - its disposition parse (`front(body, "  decision")`) misses `decision:` nested four spaces deep in schema-v2 review records.
- **Atelier links:** Atelier's `docs/product/mission-authoring.md` and `docs/product/work-model.md` link `references/tracker-records.md`, which no longer exists once this merges. ADR 0025 and `docs/strategies/agent-factory-adaptive-missions.md` name `finding-disposition.md` and `strategic-plans.md` as history. Atelier's docs are out of this item's scope, so they need a follow-up.
- **Residual readings:** the final read flagged these as two-way readings, not contradictions:
  - astromech AGENTS.md wording: the trivial base exception, unconditional "no catch-up merge", "Don't do" decision records, marking a loose end done as planning, and CI as a merge condition;
  - the light path's "approval of its head" (help);
  - starting a child before its parent has a branch. On the base it lands on the parent's branch, as tested above;
  - whether a lone author files its own loose end;
  - which branch a reviewer commits a deferral's follow-up on.
- **Maintainer row:** `maintainers/skill-evaluation.md` still names "Good Code" in one pressure-scenario row. It is not a link, and `maintainers/` is edited only to fix links.
