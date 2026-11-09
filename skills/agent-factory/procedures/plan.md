# Plan

Shape a mission's strategy, increments and first issues; `orchestrate` runs it.

## Layers

A strategy lives in a [strategy file](#strategy-file), which the mission record
links and sums up in a paragraph, or for smaller work in the mission's Outcome.
Increments are outcome-bearing slices, usually one branch and one review batch
each; in Atelier, a parent issue of its executable issues. Create issues only up
to the nearest evidence boundary, past which unfinished work, a validation
result or an open decision could change the route; later increments wait in
`backlog` with a preliminary goal, a provisional Done when and their unknowns,
and no tasks. Settle the choices the first work depends on, with `decide` where
several paths are credible, recording each per the [record
test](../references/atelier.md#records) rather than as a future issue, and plan
the work that gathers any missing evidence. Prefer a first increment that proves
a user outcome end to end over infrastructure alone. Plan shared code and APIs
docs first, the README or tutorial before the code: a bad interface is cheapest
to fix on paper.

## Shaping a Mission

A mission is one thing a person can hold in their head, plan against and finish:

- **One theme.** State the outcome in a sentence or two that explains why each
  item belongs. If it holds only by joining unrelated surfaces with "and", or
  some items fit only by stretching it, it is two missions or a mission plus
  loose backlog; a shared origin, such as one audit, is not a theme.
- **Finishable by its own work:** by the agents working it, or by asking the
  human a question they can answer now. Work waiting on calendar time, other
  repositories, people adopting a change or measuring behavior after release
  goes in a backlog item saying when to pick it up, or in a loose end if it can
  only follow a merge or release, so the mission can close.
- **Swept backlog first.** Before planning, run `groom` on the mission's area,
  plan with its proposals and answer its digest.
- **Sized to finish:** one reviewer can judge it in one debrief, usually days to
  weeks of work. Split a larger one along the outcome's seams, not item count.
- **Confident enough to run unattended.** No plan survives first contact, so do
  not settle every unknown. Ask what would most likely stall the mission or
  force a strategy change; settle each such cause that could change the outcome,
  shape or scope, and leave route-only ones for their evidence boundary. A quick
  spike is usually best: run a small one yourself, or file a larger one outside
  the mission and ask the human to approve it. Record what it shows.
- **Success criteria are optional.** Add one only for a claim about the whole
  that no single item proves, such as an end-to-end scenario. Each must be
  something the work can make true and an agent can verify at close (a test,
  scenario, measurement or named human judgment); one that restates items adds
  nothing, and one the work cannot reach stalls the mission. Propose rewriting a
  set one that fails this; do not drop it.

## Strategy File

Keep it at `docs/strategies/<mission-slug>.md` unless the repository says
otherwise, under about 150 lines, with front matter giving `status` and
`last_changed: "<date>: <summary>"`. Its sections, each left out if empty:
**Why** (two to five sentences), **Outcome** (observable), **Success Criteria**
(`- <slug>: <claim>`, each saying how it will be shown), **Shape** (key
decisions, linking ADRs and docs, and what wins when qualities conflict),
**Scope** (in, out, the target environment), **Authority** (what the
orchestrator may change without asking) and **Risks and Unknowns** (what is left
open, each with what happens if it bites). What changes with the plan (tasks,
order, status, design detail, history) belongs in the tracker, docs or Git, not
the strategy or a separate plan.

## Ready Work

A mission is ready (draft to todo) when an orchestrator could start it without
asking the human what it means: it passes the shaping tests, confidence
included, and every item fits its outcome; its first items are ready issues;
each question only the human can answer is an open decision record or a
`needs-human` item, not an issue's Unknown; and the planning has been reviewed
and the human has approved it after a walk through `atelier brief <mission>
--html` (`atelier review approve <mission> --identity <the human>`), unless the
strategy's Authority section delegates that.

An issue is ready when a worker without your context could do it from its body
alone: a Goal saying what to change and why; Done when lines, each an observable
proof that could fail if the work were wrong or missing and can hold before
merge (anything later is a [loose
end](../references/atelier.md#loose-ends-and-validation-items)); and Scope,
Context (with the target environment when it affects risk) and Unknowns where
they say something. Anything you cannot state concretely, such as "TBD", is an
evidence boundary: make it a spike, a decision or a blocked issue. Check that
the ready issues together cover the increment's outcome and agree on their
interfaces, then have the planning reviewed.

## Strategic Revision

Without the authority the core rules require, propose the revision and stop.
With it, say why replanning within the strategy is not enough, settle the choice
(with `decide` if contested), edit the strategy in place with a new
`last_changed`, say what changed, why and on whose authority in the commit and a
dated mission note, supersede any decision it replaces, and mark each affected
item valid, needing revision or obsolete.

## Handoff

Also report the strategy location, decisions recorded, increments and ready
issues created, and open questions.
