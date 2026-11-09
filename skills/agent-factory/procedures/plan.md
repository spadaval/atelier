# Plan

Make sure a mission solves a real need, has a reasonably well-defined technical
design, and is likely to succeed.

That is the planner's job. You take a mission from an idea to the human's
approval, then hand it to an orchestrator (`orchestrate`), who runs it without
the human. Each section below serves one of the three clauses and says which,
and the answers go in the mission's [strategy file](#strategy-file).

## Shaping a Mission

This section serves the real need. A mission is one thing a person can hold in
their head, plan against and finish:

- **Swept backlog first.** Before planning, run `groom` on the mission's area,
  plan with its proposals and answer its digest.
- **One theme.** State the outcome in a sentence or two that explains why each
  item belongs. If it holds only by joining unrelated surfaces with "and", or
  some items fit only by stretching it, it is two missions or a mission plus
  loose backlog; a shared origin, such as one audit, is not a theme.
- **Sized and worth it.** Size the mission on
  [sizing](../references/sizing.md)'s three scales, with a one-line reason for
  each, then write one sentence on why it is worth its cost against the cheaper
  alternative, including not doing it. These go in the strategy's **Sizing**
  section. One reviewer should be able to judge the result in one debrief,
  usually days to weeks of work; split a larger mission along the outcome's
  seams, not by item count.
- **Success criteria are optional.** Add one only for a claim about the whole
  that no single item proves, such as an end-to-end scenario. Each must be
  something the work can make true and an agent can verify at close (a test,
  scenario, measurement or named human judgment); one that restates items adds
  nothing, and one the work cannot reach stalls the mission. If one the human
  set fails this, propose a rewrite; do not drop it.

## Design

This section serves the technical design. Every mission's strategy has a
**Design** section, as long as the mission needs and no longer: the components
and where their code lives, how data flows between them, the interfaces they
expose, and each dependency chosen and why. A one-file change needs a paragraph;
a new subsystem needs more.

Settle in the design every implementation choice that is costly to reverse, such
as a framework, a crate layout or a persisted format, using `decide` where
several paths are credible and recording each per the [record
test](../references/atelier.md#records). Never leave such a choice as an item's
Unknown, where an implementer would make it alone. The design goes on the base
with the strategy as planning and is reviewed with it. Give the items that build
a component a Done when line bringing `docs/architecture/` up to date.

## Derisking

This section serves the likelihood of success. No plan survives first contact,
so do not settle every unknown; find the ones that would hurt.

- **Riskiest assumption.** In the strategy's **Derisking** section, name the
  assumption that would hurt most if it were wrong, and say how the plan tests
  it early: a spike, an interface written as docs first (a README or tutorial
  before the code, since a bad interface is cheapest to fix on paper), or a
  first increment that proves the outcome end to end. Order the items riskiest
  first.
- **Confident enough to run unattended.** Ask what would most likely stall the
  mission or force a strategy change. Settle each such cause that could change
  the outcome, shape or scope before the mission is ready, and leave route-only
  ones for their evidence boundary. A quick spike is usually best: run a small
  one yourself, or file a larger one outside the mission and ask the human to
  approve it. Record what it shows.
- **Issues only to the evidence boundary.** Increments are outcome-bearing
  slices, usually one branch and one review batch each; in Atelier, a parent
  issue of its executable issues. Create issues only up to the nearest evidence
  boundary, past which unfinished work, a validation result or an open decision
  could change the route. Later increments wait in `backlog` with a preliminary
  goal, a provisional Done when and their unknowns, and no tasks.

## Strategy File

This section serves the technical design: the strategy is the mission's design
document, which the mission record links and sums up in a paragraph. Keep it at
`docs/strategies/<mission-slug>.md` unless the repository says otherwise, with
front matter giving `status` and `last_changed: "<date>: <summary>"`. Apart from
the Design, keep it under about 150 lines. Its sections, each left out if empty:

- **Why**, in two to five sentences, and **Outcome**, observable.
- **Success Criteria**: `- <slug>: <claim>`, each saying how it will be shown.
- **Sizing**: the three scales and the worth-it sentence.
- **Shape**: key decisions, linking ADRs and docs, and what wins when qualities
  conflict.
- **Design**: the technical design.
- **Derisking**: the riskiest assumption and how it is tested early.
- **Scope**: in, out and the target environment.
- **Authority**: what the orchestrator may decide inside the mission without
  asking; never approving its plan or result, which stay the human's.
- **Risks and Unknowns**: what is left open, each with what happens if it bites.

What changes as the work runs (tasks, order, status, history) belongs in the
tracker or Git, not the strategy or a separate plan.

## Ready Work

This section serves the likelihood of success: an orchestrator must be able to
run the mission without the human. A mission is ready (draft to todo) when it
passes the tests above, every item fits its outcome, its first items are ready
issues, and nothing in its scope needs the human apart from the boundaries its
Authority reserves. Ask the human what only they can answer while you plan, and
record the answers as decisions. A `needs-human` item inside the mission's scope
fails this test. Work the mission's agents cannot finish goes outside it: a
[loose end](../references/atelier.md#loose-ends-and-validation-items) for human
or other work that can only follow a merge or release, and a backlog item saying
when to pick it up for work waiting on calendar time, other repositories, people
adopting a change or measuring behavior after release. The mission also needs
both [planning reviews](#planning-review) and the human's approval of the
[brief](#brief-and-handoff).

An issue is ready when a worker without your context could do it from its body
alone: a Goal saying what to change and why; Done when lines, each an observable
proof that could fail if the work were wrong or missing and can hold before
merge (anything later is a loose end); and Scope, Context (with the target
environment when it affects risk) and Unknowns where they say something.
Anything you cannot state concretely, such as "TBD", is an evidence boundary:
make it a spike, a decision or a blocked issue. Check that the ready issues
together cover the increment's outcome and agree on their interfaces.

## Planning Review

This section serves the likelihood of success: a bad plan costs more than any
bad change. Every mission plan gets two independent planning reviews before the
human sees it, as [Planning on the
Base](../references/atelier.md#planning-on-the-base) describes. Run the two
passes in parallel, as fresh agents that see neither your reasoning nor each
other's findings. To keep them blind, tell each not to read the other's records
or thread, and give each its own checkout of the planning commits to record in
(`git worktree add --detach`); once both finish, cherry-pick their record
commits onto the base. Use model families other than yours, and different from
each other, when available; with one family, still run both. Brief each with the
planning commits, the human's direction and this procedure, and ask it to judge
the plan against the three clauses and Ready Work. Decide the findings as
[Independent Review](../SKILL.md#independent-review) says; either reviewer
re-reviews a fix.

## Brief and Handoff

This section serves the real need: the human decides whether the need is worth
its cost. Send them the mission's brief on the site, the page `atelier brief
<mission>` names in its footer, walk them through it, and record their
approval (`atelier review approve <mission> --identity <the human>`); no
strategy can delegate it. Then hand the mission to an orchestrator,
reporting the strategy location, decisions recorded, increments and ready issues
created, the review passes and their outcome, and the unknowns left for later
evidence.

## Later Increments and Strategy Changes

This section serves the likelihood of success once the mission runs. The
orchestrator plans each later increment at its evidence boundary, within the
approved strategy and Authority, to this standard, with one independent planning
review and no brief. A change to the strategy itself goes back to a planner and
the human: say why replanning within the strategy is not enough, settle the
choice (with `decide` if contested), edit the strategy in place with a new
`last_changed`, and say what changed, why and on whose authority in the commit
and a dated mission note. Supersede any decision it replaces, mark each affected
item valid, needing revision or obsolete. Then review it as a new plan: two
passes and the human's approval.
