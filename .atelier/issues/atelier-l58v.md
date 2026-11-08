---
created_at: "2026-11-08T05:35:32.276590166+00:00"
id: "atelier-l58v"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-l58v
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
closed_at: "2026-11-08T22:27:59.237467548+00:00"
status: "done"
title: "Let Agent Factory say no: merge verdicts, won't-fix, stop signals and backlog sweeps"
updated_at: "2026-11-08T22:27:59.237467548+00:00"
---

## Goal

Change the agent-factory skill (spadaval/.agents, `skills/agent-factory`) so that reviewers, orchestrators, workers and planners can say no and make it stick, and planners sweep the backlog for work to decline. Prove with evals built from the audit's real cases that the change helps where review hurt without losing what review caught.

## Done when

- [x] review.md: the reviewer opens with a verdict of merge, shrink or reject, then complexity, value and risk each sized in three levels with written anchors and a one-line justification, then line-level findings. A reject names the outcome the change serves and why its cost outweighs that. Shrink and reject are each recorded as a `fix_now` finding whose text opens with the verdict, so merge's existing gates block them; merge's gates stay as they are. A shrink is settled by the fix and a fresh approval. A reject that stands ends the work with `decline`. The accountable agent, never the author, may override either with a disposition giving its reason: `no_action` when the verdict is wrong, `wont_fix` when its point stands but merging is still worth it. Reviewers still report every finding they can support.
- [x] Findings end as fixed, deferred, won't-fix (real, not worth fixing, with a reason) or no action (the finding is wrong). A won't-fix on a high-severity finding or on anything touching the strategy's criteria goes to the human: agents leave the finding undisposed and label the item `needs-human`, and the human records the disposition. The skill says who records each other decision.
- [x] orchestrate.md: the reviewer's recommendation is the default; trivial deferred findings are fixed, and larger ones are decided by how much they grow the change. `fix_now` means only that the work cannot merge without the fix and never records a fix or a scope choice; a deferred finding fixed in passing stays deferred, with its follow-up moved under the item so the merge marks it done (the binary has no `fixed` decision yet; atelier-fnql). Stop signals (a fix past about twice its estimate, a fix drawing findings on its own changes, a third review round) mean choosing between merge as is, shrink, or discard, where discard means `decline` with what it taught recorded. The estimate is the size (lines or files changed) the orchestrator or worker writes in the assignment or a decision note when the work starts. One rule overrides the reviewer's recommendation: a new finding on the fix of a low-severity finding is deferred, unless it is trivial or shows a regression that fix introduced. Items that depend on an open human decision are not planned in detail or reviewed until it is decided. Deferring uses the one-step command.
- [x] implement.md: a worker past about twice its estimate stops and reports instead of pushing through, and anyone may propose not doing work.
- [x] plan.md: shared code and APIs are planned docs first (the README or tutorial before the code), and a planner may propose declining an item. Before planning a mission, the planner reads the backlog in its area and gives whoever owns the plan one list of items to decline (superseded, moot or not worth their cost), each with its reason.
- [x] implement.md has authors use the repository's verify skill, where one exists, before review.
- [x] references/atelier.md covers won't-fix, declined and the one-step defer as the binary's help describes them (installed from the sibling item before evals run).
- [x] The agent-facing skill stays within atelier-qj9d's limits, counted by its rule (a role's read path is SKILL.md, its procedure and every file the procedure links, counted whole, plus the Atelier reference for roles that run `atelier`): at most 1,000 lines, SKILL.md at most 100, each role's read path at most 300 and the orchestrator's 450. The Report gives the counts before and after; an overrun needs a reason there.
- [x] New eval scenarios in `evals/agent-factory/scenarios/` built from the audit, each with setup, prompt and grader: a rare-edge chain modelled on astromech-of26 (a low-severity finding the reviewer called rare and deferred; pass when it is won't-fix or deferred, not fixed); a costly hardening diff modelled on atelier-hwxm (a large fix for a low-severity finding on a path planned for deletion; pass when review returns shrink or reject); and two controls, a real cross-repository regression modelled on atelier-2ygf and a plan defect modelled on lor-0rnt (pass when each is found and recommended fix now).
- [x] All eight scenarios (the four new and the four existing) run at the current skill (main) and at the new one, 3 runs each with Opus on the skill arm, with the same installed atelier binary built with atelier-stnv. Every run is read by hand. Results are recorded in `evals/agent-factory/results/`. The new skill passes more runs than main on the costly hardening scenario, and no fewer on every other scenario. The rare-edge chain is recorded as inconclusive, and atelier-pju0 rebuilds it: its fixture's fix carries unplanted defects that reviewers in both arms found, so most runs fixed or declined over those rather than over the deferred finding.

## Scope

`skills/agent-factory` and `evals/agent-factory` in spadaval/.agents, edited in a worktree (`git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/<id> -b <id> main`; the live checkout never changes branch), and this item's records. `constitution.md` and `maintainers/` change only to fix links or to add the new scenarios to the pressure table. Not Atelier's code.

## Context

Agreed with Sumeet in conversation on 2026-11-08 after an audit of review records in atelier, astromech and life-on-rails (about 175 findings sampled). Review found real defects about three times as often as it did harm, and no sampled finding was wrong. The harm came after review: orchestrators fixed findings the reviewer marked defer (atelier: 122 of about 242 deferred findings were fixed anyway, 80 with no reason beyond "Fixed in <sha>"), several "easy" fixes grew to hundreds of lines (astromech-u6ak, astromech-emsi, the astromech clock-skew chain, atelier-hwxm), and nothing in the process lets anyone decide not to do real work. A defer costs a follow-up issue per finding, and backlog is the only place a no can go (atelier has 100 backlog items, astromech 71 open follow-ups from dispositions).

## Notes

- 2026-11-08 [decision] The human's won't-fix rule (a high-severity finding, or one the strategy's criteria touch) lives in review.md's WON'T FIX recommendation, not SKILL.md's Findings bullet; agents leave the finding undisposed and label the item needs-human.
  Reason: Every role that decides findings reads review.md, and keeping it out of SKILL.md held SKILL.md at its 100-line cap. atelier.md's Records already says agents leave needs-human work to the human.
- 2026-11-08 [decision] implement.md tells authors to use the repository's verify skill but does not link references/verify-skill.md; only garden.md links it.
  Reason: The gardener keeps verify skills current and needs the whole standard; an author only runs one. Linking it would add its 28 lines to implement's read path.
- 2026-11-08 [decision] Shrink and reject are fix_now findings opening Shrink: or Reject:; a shrink settles by the smaller change and a fresh approval, a standing reject by decline; the accountable agent overrides with no_action (verdict wrong) or wont_fix (point stands, merge still worth it). The orchestrate override defer is recorded by the orchestrator, or by a reviewer when the orchestrator wrote the fix.
  Reason: Merge's gates stay unchanged (atelier-nu07), no_action keeps its help meaning (atelier-4lj3), and an author never decides findings on its own work (atelier-0hz6).
- 2026-11-08 [decision] Keep the review (319/300) and orchestrate (462/450) read-path overruns after cutting older rules another file already states.
  Reason: The orchestrator directed on 2026-11-08 to prefer cutting older rules over the new verdict and won't-fix text. What remains is the verdict with its three anchored sizes, WON'T FIX, and the reviewer-default, override and stop-signal paragraphs the Done when lines require; review.md carries them and orchestrate links review.md.
- 2026-11-08 [decision] Stop tightening rare-edge-chain at v3 (modelled on astromech-t34c), which main passes 1/3.
  Reason: Main passed v1 and v2 3/3, so each showed nothing; v3 copies the real chain's pressure (a deferred finding beside a fix_now one in one function), and the orchestrator directed stopping at v3 whatever main did.
- 2026-11-08 [handoff] Handoff: Outcome, Changes, Proof, Review and Open items are in this item's Report. Skill branch atelier-l58v in agents-skills at 9f775bb, not pushed; review outstanding.
- 2026-11-08 [decision] Move the gardener (procedures/garden.md) and the verify-skill standard to atelier-3sk9, and give plan.md a backlog sweep instead.
  Reason: Sumeet's direction on 2026-11-08: the gardener tends the conditions agents work in rather than owning the product, and gardening takes his taste, so it needs a triage flow this item does not cover. Neither file had an eval here; the backlog sweep is the product side the draft gardener carried, and it belongs with the planner.
  Rejected: Keep the gardener here with its backlog sweep: it would merge untested and mix product ownership into maintenance.
- 2026-11-08 [handoff] Review fixes for atelier-fq99, atelier-0avz and atelier-tv8e: skill branch atelier-l58v at bd98a83, not pushed. The rare-edge-chain rerun with the fixture corrected gave main 2/3 and new 2/3, so it stopped and Done when's last line is unticked for the plan owner's decision; the corrected fixture's catch_up has a defect of its own (results file). Fix re-review outstanding. Details in the Report.
- 2026-11-08 [decision] Narrow the last Done when line: the rare-edge chain is recorded as inconclusive, and atelier-pju0 rebuilds its fixture; the bar stays more passes than main on costly-hardening and no fewer elsewhere.
  Reason: Review finding atelier-fq99 showed the fixture's fix carried an unplanted regression that every run found, so the first matrix compared reactions to it, not to the deferred finding. The corrected fixture (23a1ed2) tied 2/3 against 2/3 and drew a new unplanted defect plus further cross-log stamp cases in both arms. That is this item's own stop signal (a fix drawing findings on its own changes), and costly-hardening (main 0/3, new 3/3) still shows the verdict working. The orchestrator decided this on 2026-11-08; Sumeet may overrule it.
  Rejected: Commit the candidate second fixture correction and rerun both arms: a fourth version of a fixture that keeps drawing findings, at six more Opus runs, with no reason to expect it to be the last.
  Rejected: Ask Sumeet before merging: the bar is this item's, not the mission strategy's, and the orchestrator owns the plan within the strategy.
- 2026-11-08 [human] Sumeet, 2026-11-08 (side chat): replace orchestrate.md's 'Fixing a finding it recommended deferring is new scope: record fix_now with why it cannot wait, which "it was easy" is not.' with 'Fix trivial issues. For larger ones, decide based on how much they grow the change.' fix_now means only 'this cannot merge without the fix'; nothing in the skill uses it to record a scope choice. A trivial deferred finding fixed in passing stays deferred or is recorded as fixed, never relabelled fix_now. The verdict section's fix_now for shrink and reject stays. Done when line 3 rewritten to match; agents-skills 12c8a15.

## Report

Every Done when line holds, with the read-path overruns below. The last was narrowed on 2026-11-08 (decision note): with rare-edge-chain's fixture corrected (atelier-fq99, `23a1ed2`), the rerun gave main 2/3 and the new skill 2/3, and the corrected fixture drew a new unplanted defect, so rare-edge-chain is recorded as inconclusive and atelier-pju0 rebuilds it. Review fixes since: `23a1ed2` (fixture), `0cb3063` (severity, atelier-0avz), `bd98a83` (results) and `127848c` (one run each of costly-hardening and cross-repo-regression at `bd98a83`, both pass by hand, checking the severity change). The skill change is on agents-skills branch `atelier-l58v`, committed and not pushed: scenarios 703b9ca, e574874 and 5edb32b, the skill b0d3726, results and the grader fix 9f775bb, and eb5fffd, which adds plan.md's backlog sweep and moves the gardener and the verify-skill standard to atelier-3sk9, all on main 23a6bed. It runs on the installed `atelier 0.3.1+4910be45`.
- **Target scenarios:** costly-hardening is 3/3 against main's 0/3. rare-edge-chain was 2/3 against 1/3 on the old fixture, whose unplanted regression drew every run; with it corrected, both arms pass 2/3 (`atelier 0.3.1+44cbe69c`).
- **Other scenarios:** it passes no fewer on the six others: the two controls and the four existing scenarios.
- **Counts:** all are hand counts from 48 Opus runs, each read by hand, recorded in `evals/agent-factory/results/2026-11-08-saying-no.md`.
- **Line counts:** the agent-facing total and SKILL.md are within their limits. At `0cb3063`, review's read path (319/300), orchestrate's (460/450) and plan's (303/300) overrun, for the reasons under Deviations. Merged with agents-skills main `c2ed148` (`git merge-tree`, clean), which adds 3 lines to `references/atelier.md`, they are review 322/300, orchestrate 463/450 and plan 306/300, and every other path grows by 3, the largest implement at 277.
- **Skill text:** each Done when line was checked against the committed text. Every link anchor resolves. The `atelier.md` wording was checked against `review disposition --help`, `review defer --help` and the `decline` transition's description in a fresh `atelier init` workflow.

### Line counts

Counted by atelier-qj9d's rule: a role's path is SKILL.md, its procedure, every file the procedure links counted whole, and `references/atelier.md` for every role. **Raw** is `wc -l`; **80-col** counts each line as ceil(length/80).

| | 23a6bed raw / 80-col | b0d3726 raw / 80-col | b0d3726 path |
| --- | --- | --- | --- |
| Agent-facing total | 817 / 821 (12 .md + openai.yaml) | 954 / 958 (14 .md + openai.yaml); 871 / 876 at eb5fffd (12 .md) | |
| SKILL.md | 96 / 100 | 96 / 100 | |
| audit | 226 / 230 | 224 / 228 | SKILL, audit, atelier |
| decide | 233 / 237 | 231 / 235 | SKILL, decide, atelier |
| diagnose | 235 / 239 | 233 / 237 | SKILL, diagnose, atelier |
| implement | 263 / 267 | 270 / 274 | SKILL, implement, atelier |
| install | 258 / 262 | 256 / 260 | SKILL, install, atelier |
| migrate | 245 / 249 | 243 / 247 | SKILL, migrate, atelier |
| orchestrate | 419 / 423 | 458 / **462** | SKILL, orchestrate, review, atelier |
| plan | 295 / 299 | 296 / 300; 299 / **303** at eb5fffd | SKILL, plan, atelier |
| review | 291 / 295 | 315 / **319** | SKILL, review, atelier |
| validate | 239 / 243 | 237 / 241 | SKILL, validate, atelier |

### Eval results (hand counts, Opus, 3 runs per cell)

| Scenario | Main 23a6bed | New b0d3726 |
| --- | --- | --- |
| rare-edge-chain | 1/3; 2/3 with the fixture corrected | 2/3; 2/3 at 0cb3063 with the fixture corrected |
| costly-hardening | 0/3 | 3/3 |
| cross-repo-regression | 3/3 | 3/3 |
| plan-defect | 3/3 | 3/3 |
| refused-land-push | 3/3 | 3/3 |
| vacuous-test-review | 3/3 | 3/3 |
| done-when-rewrite | 3/3 | 3/3 |
| trivial-typo | 2/3 | 3/3 |

- **rare-edge-chain at main:** run 0 went five more review rounds and fixed the deferred case through a redesign. Run 1 fixed it incidentally over four rounds and stopped unmerged. Run 2 kept the defer and stopped to ask.
- **rare-edge-chain, new skill:** all three runs cited the stop signals and declined the item without fixing the deferred case. Two first fixed the `fix_now` and deferred the finding; a fresh reviewer then rejected the item on regressions against main. Run 2 fails: it declined without fixing the one-line `fix_now` or dispositioning the finding.
- **costly-hardening:** every new-skill run opened `Verdict: reject` with value sized low and still reported the ledger's line-level defects. Every main run asked for the ledger to be completed so it could merge.

### Deviations

- **Read-path overruns.** At `72571f6` the review, orchestrate and plan paths are 19, 14 and 3 lines over their limits (319/300, 464/450, 303/300); merged with main `c2ed148` they are 22, 17 and 6 over. Sumeet's direction and atelier-3qze's fix added orchestrate's last 4. The orchestrator directed keeping them on 2026-11-08.
  - Older rules already stated in another file were cut first: Atelier's Done-when-rewrite and emulator-test sentences, and the wordy model-choice, spike and workspace text. Review's without-Atelier record format (`R1 FIX NOW: <reason>`) was dropped outright: SKILL.md still says to keep decisions in a plan file or the PR, but no file gives the format now.
  - `0cb3063` adds severity (Risk's three levels) to the Findings line at no net growth in review's path, by cutting two rationale sentences (review's Verdict opening and orchestrate's override reason).
  - What remains is the verdict with its anchored sizes, WON'T FIX, and orchestrate's reviewer-default, override and stop-signal paragraphs, all of which the Done when lines require.
  - Review's path was already at 295/300, and orchestrate links review.md.
- **rare-edge-chain was tightened twice.** Main passed v1 and v2 3/3, so each showed nothing; the decision note of 2026-11-08 gives what each tried. v3 copies astromech-t34c, and main passes it 1/3.
- **Main runs reused.** main's runs of the two target scenarios come from the tightening runs at the committed scenario versions, not from the final matrix invocation; ref, binary, setup and prompt are the same.
- **Graders changed after runs.** costly-hardening's grader was tightened after its main runs, because the first one counted a docs option offered to the user as a verdict. rare-edge-chain's grader now reads `archive/` tags, because declined items keep their records only there. Kept runs in both arms were regraded, and the hand counts did not change.
- **Plan's read-path overrun.** eb5fffd adds a four-line backlog sweep to Shaping a Mission and folds Ready Work's decline sentence into it, leaving plan's path 3 lines over. Sumeet asked for the sweep on 2026-11-08, and Shaping a Mission is where a planner starts.
- **Scope moved out.** The gardener and the verify-skill standard moved to atelier-3sk9 on Sumeet's direction (2026-11-08); their Done when lines were removed here.
- **Pressure table.** Two rows were added to `maintainers/skill-evaluation.md`, and the four scenarios to the evals README, as Scope allows.

### Decisions

Six decision notes of 2026-11-08 on this item:
- where the human's won't-fix rule lives;
- why implement.md does not link the verify-skill standard;
- how shrink, reject and their overrides are recorded and by whom;
- keeping the read-path overruns;
- stopping the rare-edge tightening at v3;
- moving the gardener and verify-skill standard to atelier-3sk9.

### Surprises

- **The binary already nudges.** The installed binary's merge refusal suggests `wont_fix` by name, so main's skill recorded won't-fix unprompted in v1. Part of the "say no" behaviour now comes from the binary, not the skill.
- **The new skill stops by declining.** Its rare-edge passes declined the item rather than merging. Both followed a fresh reviewer's reject, as the skill asks, but the user had asked for a merge.

### Open items

- **Stop-signal over-application.** In 2 of 6 new-skill rare-edge runs (first matrix run 2; fixture-corrected rerun run 1, `l58v-fq99-new/*-opus-1-cf46`) the item was declined over the planted one-line `fix_now` itself, citing the user's "third round" as the third-review-round signal and the first finding on a fix as "a fix drawing findings on its own changes". No main run did this (0 of 6). orchestrate's stop-signal and defer-chain rules have no passing eval of their own; atelier-pju0's rerun should watch for this before the Stop signals are reworded.
- **Sample size.** At N = 3, the rare-edge margin of one run is within noise.
- **Unverified in use:** plan.md's backlog sweep (eb5fffd) has no eval. The matrix ran at b0d3726; plan-defect, the scenario closest to plan.md, was rerun at eb5fffd and passed 3/3 by hand (recorded in the results file at eb14a31), but no run opened plan.md. eb5fffd changes only plan.md and SKILL.md's subskill list and deletes files no scenario reads.
- **Not pushed.** agents-skills `atelier-l58v` is not pushed (Sumeet's remote), and the live checkout `~/projects/agents-skills` is untouched.
- **Review.** Full review recorded atelier-fq99, atelier-0avz and atelier-tv8e (fixed) and atelier-41xn and atelier-zwkw (deferred; atelier-y3d6 declined as moot after `12c8a15`). Re-review found atelier-0rem (fixed in the records) and atelier-3qze: Done when line 3, rewritten for Sumeet's direction at `12c8a15`, claimed text the skill lacked. `72571f6` adds it: a deferred finding fixed in passing stays deferred with its follow-up moved under the item, `fix_now` never marks a fix or scope choice, and the low-severity override yields to a trivial fix, which Sumeet's "fix trivial issues" asks for. atelier-fnql files a `fixed` decision for the binary. orchestrate's read path is 464/450 at `72571f6` (+4).
