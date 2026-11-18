---
schema: atelier.mission
schema_version: 3
id: atelier-kw8u
title: 'Measure the lead, implementer and reviewer loop: evals that grade outcomes, run as the hosts run, and stop rotting'
status: open
labels: []
lead: null
planners:
- 0b3307d5-85cd-4102-be6c-51da3b535c89
created_at: 2026-11-18T20:03:47.624288179Z
---

## Why

The human asked on 2026-11-18, in conversation, to modernise the Agent
Factory evals, take a hard look at their cases and revisit their premise.
They set the focus the same day: the core loop of lead, implementer and
reviewer. They also said the lead rule is drawn in the wrong place: work
that needs review goes to an implementer, and the rest is the lead's.

The evals ask one question: does the skill beat no skill under process
pressure? Four things are wrong with what they measure today.

- **The runs are not set up as the hosts are.** Every cell is one model in
  every role, at the harness's default effort. The work host does run
  `gpt-6.1-sol` in every role, at `high`. This host runs Fable as lead, Opus
  as implementer and Sol at `xhigh` as reviewer, and no run has ever had a
  reviewer from another lab than its implementer.
- **A pass means the process was followed, not that the work came out
  right.** `next-settle-within-authority` fails a lead for writing any file
  outside `.atelier/`, whatever the file. Time and tokens are recorded and
  never reported as a result.
- **The reviewer has two toy cases.** The control pilot of 2026-11-10 found
  that review depth, not the machinery around it, is what separated good
  changes from bad. The set has one 20-line and one 200-line review, both
  invented.
- **The grading is heavy and still wrong.** Graders and their tests are
  3,106 lines that match wording with regular expressions and parse shell
  commands. In the smoke run of 2026-11-17 three of nine grades were wrong
  or null, and all nine runs passed by hand. Three runs per cell is noise:
  `gpt-6.1-sol` with no skill on `next-refused-push` read 1 of 3 on
  2026-11-11 and 3 of 3 on 2026-11-17.

Two cases no longer test anything. `next-done-when-rewrite` says its 50 ms
line cannot hold, and every model since 2026-11-11 has met it by building an
index. `next-rebuttal-kept` passes 3 of 3 with no skill on every model, its
command was removed, and its pass (do not take the kept finding's fix)
contradicts `roles/implementer.md`, which says to fix it.

The fixtures themselves are sound again: item atelier-rt3o brought them to
the branch model and CI's `evals` job sets up and grades each one with no
model.

## Outcome

The evals answer three questions about the loop, and each result says which
one it answers.

- **Does an agent get through the loop on what the binary prints?** The
  loop cases: an item from ready to merged, a round of changes, a refused
  push, a lead running a mission.
- **Does a review catch what matters?** The review cases: changes from this
  repository's own history that passed review and carried a defect found
  after the merge, replayed on today's binary, beside the two invented ones.
- **Does the skill change either?** The skill is an arm run over both, not a
  suite of its own.

A matrix runs in a named setup, a model and an effort for each role. Two
setups are kept: `sol`, the work host's, and `house`, this host's. Each
result gives passes, time and tokens for every cell.

A grade has two parts. Code reads facts from the records and the tree: what
merged, who approved it, whether the tests pass. A model from another lab
than the run's agent reads the run against the case's rubric for what
depends on meaning: whether a finding says the test cannot fail, whether a
review asked for less. No grader matches wording or parses a shell command.

The lead's line is the one the human gave. A lead does itself what needs no
review: records, and a change whose correctness is evident from its diff.
Everything else goes to an implementer. The cases grade that line from the
records, in both directions.

## Success criteria

- setups: one command runs the matrix in `sol` and in `house`. In `house` a
  review is written by a model from another lab than the implementer's, and
  every result names its setup.
- no-null-grades: over one full matrix in each setup, no run is graded
  `null` for a reason other than an invalid run, no grader holds a regular
  expression over a review's or a reply's wording, and none parses a shell
  command to learn what it wrote.
- judge-holds: 30 runs of that matrix, drawn across cases and both verdicts,
  are read by the thread that wrote the results, and the judge's verdict
  differs from that reading on at most two. Each difference is listed with
  the run.
- lead-line: the two lead cases and the mission case run with
  `roles/lead.md` before and after its change, in both setups. The results
  give both, and the new text stays if no cell passes fewer runs with it.
- reviews-from-history: at least three changes from this repository's
  history whose defect was found after the merge are replayed as review
  cases on today's binary. The results give, for each setup's reviewer, how
  many of the known defects its review found, how many fix-now findings the
  judge could not support, and the time and tokens of each review.
- every-case-earns: the README says for each case which of the three
  questions it answers and what its last result was in each setup. A case
  that passed every run of every cell is cut, or the README says what it
  guards.
- cost-shown: every results table gives time and tokens per cell beside its
  passes.
- still-gated: `evals/agent-factory/test.sh` sets up and grades every case,
  new ones included, with no model, and CI's `evals` job runs it.

## Scope

- In: `evals/agent-factory/`; `skills/agent-factory/roles/lead.md` and the
  lines the binary prints to a lead, in `crates/atelier-core/src/say.rs`;
  the places that name a scenario, in `docs/design.md`,
  `.github/workflows/ci.yml` and `scripts/ci-jobs.sh`.
- Out: evals of the planner, the gardener, the backlog procedure and the
  readiness checklist, which are conversations with the human, and the
  README says they are unmeasured; running the evals inside a T3 Code
  instance; a model call in CI; any other change to the skill's text; the
  fixes the smoke run filed (atelier-u49o, atelier-yhaf, atelier-jfos); the
  scorecard (atelier-c1z3, atelier-pwhn), which measures real use.
- Target environment: this host, with Claude Code and Codex signed in. The
  repository is public, so a case holds only code from this repository.

## Authority

The lead may choose case names, fixtures, rubric wording, the judge's model
where it is from another lab than the run's agent, the number of runs per
cell, which defects from
`docs/architecture/quality/scorecard-baseline/defects.md` are replayed, and
the order of items within their blockers. It may cut a case that cannot be
made to tell two arms or two setups apart, saying so in the README. It may
add an item that extends the review cases past three once atelier-kw8u's
first replay item has reported what one review costs.

It may not change the skill's text outside `roles/lead.md`; keep the new
lead text where it passes fewer runs than the old, without the human; put
code from another repository in a case; raise the limit of two eval agents
alive at once; or build a case that needs a harness feature of its own. A
case is a fixture, facts and a rubric, and one that needs more is cut. The
human set no limit on what a matrix may cost, and asked on 2026-11-18 that
the evals not grow complex.

## Result
