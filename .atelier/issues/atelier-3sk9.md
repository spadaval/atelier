---
created_at: "2026-11-08T21:02:40.598319646+00:00"
id: "atelier-3sk9"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-3sk9
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9s9i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add a gardener: agents fix routine mess, and bring the human the judgment calls"
updated_at: "2026-11-09T00:07:09.297053584+00:00"
---

## Goal

Give Agent Factory a gardener (`procedures/garden.md`) that tends the conditions agents work in, not the product. It finds where agents trip, from their transcripts and the code, and fixes the ground with guardrails, weeding and better signage. The human gardens too: the agent does the bulk reading and the routine upkeep, and brings the human anything that takes taste as a recommendation to triage.

## Done when

- [x] Boundary: garden.md says the gardener changes how agents work here (code structure without behaviour change, checks, docs, error and help text, skills, tooling), never what the product does, and sends a product question it meets to the plan's owner as an issue.
- [x] Finding trips: a run reads agent transcripts and the code, plus review records, reverts and failed checks. It looks at what is new since its last run (the last 7 days on a first run). A trip in the transcripts earns a new rule, check or gate only once it has happened at least twice in the last 30 days; mess found by reading the code (dead code, residue, a broken link) needs no repeat. Its transcript method works for any harness: start from raw transcripts, not summaries or memory; check the transcript around each claimed failure and handoff; follow subagent and thread lineage to the root.
- [x] Fix level: each trip is fixed at the highest level that works: architecture (make it impossible), then a check, then a sign where the agent trips (an error message or help text before a rule in a distant doc). Guardrails are nudges: a better default or message comes before a new gate, and removing a rule whose mistake can no longer happen counts as much as adding one.
- [x] Triage: garden.md splits the gardener's changes into routine upkeep and recommendations, with examples of each. Routine upkeep is a change whose correctness is evident from the diff and that adds or removes no rule, check, gate, interface or default, at most about 100 changed lines (dead code, residue, a doc line the code contradicts, a broken link): the gardener makes it, has it reviewed by an agent like any change, and merges it. Everything else, and anything the gardener is unsure of, is a recommendation.
- [x] Recommendations reach the human as one digest per run and repository: an item labelled `needs-human` whose entries each give the trip with its transcript or code evidence, the proposed fix and its level, its size and the alternatives. The human answers each entry (do it, do it differently, no) in a note on that item. The next run reads the answers: accepted entries become issues, it adds to an unanswered digest instead of filing another, and it does not propose a declined recommendation again without new evidence.
- [x] A run covers one repository, never touches a branch in flight, and reports the window, the trips found, the routine fixes merged, the recommendations made, and the rules and checks added and removed.
- [x] `references/verify-skill.md` sets the verify-skill standard, starting from atelier-l58v's draft, which the Design section below copies. The gardener keeps a repository's verify skill working when agents trip on it; authors keep its feature map current with their features.
- [x] SKILL.md lists `garden`, and garden's read path stays within atelier-qj9d's 300 lines, counted by its rule; the Report gives the count.
- [x] Trial: a fresh agent given only the skill runs the gardener on this repository, and its output is read by hand. Each trip it reports is checked against its transcript, no change it calls routine should have been a recommendation, and its digest is in the Report. The trial proposes removing at least one rule or check, or says why it found none. The trial merges and files nothing.

## Scope

`skills/agent-factory` in this repository, after atelier-dkul moves it here, and this item's records. Not the daily schedule (atelier-jy8k).

## Context

Agreed with Sumeet in conversation on 2026-11-08. atelier-l58v's draft gardener (agents-skills 9f775bb, `procedures/garden.md`) mixed in product work: sweeping the backlog and judging whether merged changes were worth their cost. Those went to plan.md's backlog sweep and review's verdict, and the gardener moved here. Gardening is also Sumeet's job, and much of it takes taste, so the agent reads and does routine upkeep and brings the rest as recommendations. atelier-9s9i folds codex-reflect's transcript method into audit.md first; this item moves or links it, whichever fits.

## Unknowns

- Where each harness on this host (Claude Code, Codex, T3 Code) keeps transcripts, and how much a run can read in one session.

## Design

atelier-l58v's draft verify-skill standard, copied from spadaval/.agents `git show 9f775bb:skills/agent-factory/references/verify-skill.md` (an unpushed branch whose next commit deletes it):

> A repository's verify skill lets any agent prove that the app works, not only
> that its tests pass. It is a skill in the repository (for example
> `.agents/skills/verify/SKILL.md`) built on two things:
>
> - **A control CLI** the agent drives the app with: commands that start it, act
>   on it as a user would (open a page, send a request, tap a button) and read its
>   state as text. Output an agent must interpret by eye, such as a screenshot,
>   is a last resort.
> - **A feature map:** each thing the app does, and the command or scenario that
>   proves it works. A change names the features it touches, and its proof runs
>   their entries.
>
> Its sections:
>
> - **Launch:** start the app from a clean checkout with its dependencies in one
>   command, and how to tell it is ready.
> - **Doctor:** check the environment first (tools, ports, credentials, devices)
>   and say how to fix each failure, so a broken setup is not taken for a broken
>   change.
> - **Drive:** each control CLI command, with an example.
> - **Evidence:** what to capture as proof (command output, screenshots,
>   recordings) and where, so a reviewer can check it.
> - **Cleanup:** stop what Launch started and remove what the run made.
>
> Nobody relies on it until an agent has run it end to end from a clean checkout:
> Doctor, Launch, Drive of one feature, Evidence and Cleanup.

## Report

Agent Factory now has a gardener. `procedures/garden.md` finds where agents trip, from transcripts, review records, failed checks and the code. It makes routine upkeep itself and brings everything else to the human as one `needs-human` digest per run. `references/verify-skill.md` sets the verify-skill standard from l58v's draft, and `references/transcripts.md` holds the transcript method, moved out of audit.md so both procedures link one copy. Two trial runs on this repository, each by a fresh Opus agent given only the skill, filed and merged nothing. The first, at d82576b9, shaped the text, and the second ran the final text at 7f26fee2 (Trial 2, below). I checked the trips each cited against its transcripts.

Read paths, counted by atelier-qj9d's rule (SKILL.md, the procedure, every file it links, plus references/atelier.md), master 6772ac6f, then this branch:

| Role | Before | After |
|---|---|---|
| garden | none | 300 (SKILL 98, garden 60, atelier 105, transcripts 18, verify-skill 21) |
| audit | 243 | 253 |
| implement | 274 | 276 |
| groom | 297 | 298 |
| plan | 302 | 303 |
| review | 323 | 324 |
| orchestrate | 454 | 455 |
| decide, diagnose, install, migrate, validate | 235, 237, 261, 247, 241 | +1 each |

SKILL.md's one added line, for listing `garden`, raises every role by 1. plan, review and orchestrate were already over their limits on master. The agent-facing skill is 1,069 lines, up from 977, past qj9d's 1,000. This item's Done when sets no total, so the overrun is reported here and not fixed.

### Trial 1 (d82576b9)

The trial window was 2026-11-01 to 2026-11-08, with repeats counted over 30 days. It read 19 root Claude Code sessions with their subagents, 3 Codex sessions (Codex keeps only today's), 161 review findings and 5 failed CI runs. I checked the trips with `grep` in the cited files:

- **/tmp quota.** `land`, `start` and `merge` fail when /tmp is full: "Disk quota exceeded" in atelier 8624a966 (5 hits) and 9fa7eab8, astromech ceea6cb2, and life-on-rails d2903f71. The cause is `tempfile::tempdir()` at `commands/git.rs:119`, `record_guard.rs:107` and `merge.rs:200`. This is R1.
- **Master checked out elsewhere.** "is already used by worktree" in a0cc9335, c5819871, 71876b69, fa0af964, 9fa7eab8 and 1107057e. `~/projects/atelier` is detached behind master because master is checked out in a T3 worktree. This is R2.
- **Trailing whitespace from hand-edited records.** Master CI run 42331331319 failed on it ("Record approval atelier-hfbr", checked with `gh run view`). This is R3.
- **Follow-up refused on the work branch.** A follow-up filed on master is refused on the work branch without naming `merge --sync`: fa0af964 ("it exists only on branch master") and dc07e8bf (atelier-1nlr). This is R4.
- **Dropped trips.** The trial dropped three trips with reasons: a refusal that already says what to do, harness `sleep` blocking, and an old binary's `start --base`.

The routine fix is commit 7d0edba1 on the local branch `garden-trial`, with 3 lines added and 6 removed. It removes `docs/architecture/quality/agent-factory-atelier-validation.md`'s advice to run `atelier check --fix`, and its mention of export and rebuild diagnostics. All three were removed in 24af0ee5 and 5df9fa56, and `atelier check --help` has no `--fix`. The fix adds and removes no rule, so it is correctly routine. It is unreviewed and unmerged.

The digest it would file is "Garden atelier: 5 recommendations" (decision, `needs-human`, `garden`). Each entry gives the trip, its evidence, the fix's level, its size and the alternatives:

- **R1, architecture.** Make snapshot and scratch directories inside the repository, as `start` (`.atelier/runtime`) and merge's checkout already do. About 10 lines.
- **R2, host layout.** Keep master checked out in `~/projects/atelier`. The alternative is a sign: `status` says how far behind a detached checkout is, and where the base is checked out.
- **R3, check.** `atelier check` warns about trailing whitespace in records.
- **R4, sign.** The refusal names `atelier merge <owner> --sync`.
- **R5, removal.** Plan atelier-hfme now (drop `start --base` and the `branch_base` write), and possibly the `prime` alias.

Asked whether it looked for rules or checks to remove, the trial added **R6, removal**. It drops `cargo fmt -- --check` and `git diff --check` from `merge_checks` in `.atelier/workflow.yaml`, because `scripts/merge-ci-passed.sh` already requires a CI pass on the same code, and CI runs both checks. I confirmed that by reading the policy, the script and `ci.yml`. It keeps `cargo metadata --loc### Trial 2 (7f26fee2, the final text but for three small fixes)

Run at review finding atelier-m2zn's request, from a fresh checkout of master. It covered the same window, 20 root sessions and their 173 subagents, 161 findings and 2 failed CI runs. It made one routine fix (60a3e1d4 on local branch `garden-trial2`), the same `check --fix` doc fix as trial 1. It wrote "Garden atelier: 8 recommendations", and filed it in a throwaway Atelier repository, where `issue show --full` listed all 8 `### R` entries under Options and `atelier check` passed. I checked these by `grep` in the cited transcripts:

- **R1, `issue create --parent <mission>` refused.** a0cc9335: "'task/' is not a valid branch name".
- **R4, removal.** `SKILL.md:48` "its PR is not marked ready" outlived the no-draft-PRs direction. 9fa7eab8 has 2 `gh pr create --draft` calls after it.
- **R5, free text starting with `--` parsed as a flag.** 7377a49f: "first summary started with `--binary`, so it was parsed as a flag".
- **R8, removal.** A stale memory note on this host, since corrected by hand (memory, not the repository).

The other entries are the detached main checkout (R2), the follow-up refusal (R3), docs and help left stale by changes (R6, 12 of 99 `fix_now` findings) and snapshots unpacked into a full $TMPDIR (R7). Every entry is a recommendation, which is what garden.md now says for a repository that builds the tools agents trip on. It proposes two removals (R4, R8).

Three of its notes were fixed after it, within the same 300 lines: the window goes in the digest's Context, a refusal followed is a gate working, and gardener runs are left out of the transcripts read. These are left open:

- where a host or memory fix goes, other than as a digest entry;
- whether the twice-in-30-days rule applies to evidence from review records;
- forked T3 threads copying their parent's events, so counts double;
- how far to read other repositories.

ked`, since CI builds without `--locked`.

### Deviations

- The trial found seven gaps in garden.md, which I fixed after it ran, so it ran the version at d82576b9:
  - the product boundary when the repository builds the tools agents trip on;
  - trips in other repositories on the same tool;
  - a single fixed review finding not counting as a trip;
  - pointing an entry at an existing item;
  - host-setup fixes;
  - where entries go in a decision record (`### R1` under Options; it was refused until it used them);
  - transcript stores that keep only recent sessions.

  Trial 2 ran the fixed text.
- The trial's first report proposed no rule or check removal and did not say why. It added R6 when asked.

### Open items

- The trials' routine fix and digests are not applied, because a trial files and merges nothing. Once this merges, a real run can apply the fix and file the digest.
- Trial 2's four unclear points, listed above.
- The skill's 1,069 lines are past qj9d's 1,000 total.

## Notes

- 2026-11-08 [decision] Estimate: about 200 changed lines across garden.md, verify-skill.md, a transcripts reference, SKILL.md, audit.md and implement.md, records excluded.
  Reason: implement.md asks for a size estimate when the assignment gives none.
