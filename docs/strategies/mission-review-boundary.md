---
status: draft
last_changed: "2026-11-08: fix planning review findings; Sumeet dropped read-path limits, kept later increments inside the mission"
---

# Why

A mission exists so the human can review a batch of related work twice: once
as a plan before any of it starts, and once as a result after all of it has
merged. Agent Factory never says so. Nobody owns planning: `plan.md` is a
standard the orchestrator, `groom` and whoever is talking to the human apply in
passing, so missions reach approval with no technical design, no sizing and no
plan for their riskiest assumption (missions atelier-1gb3 and atelier-yfqu
both did). Planning gets one review pass, though a bad plan costs more than
any bad change. And the orchestrator's stop rule lists four exceptions instead
of one plain instruction, so agents bring the human questions mid-mission that
planning should have settled.

# Outcome

Agent Factory treats a mission as a human review boundary. A planner role
takes a mission from idea to the human's approval: it makes sure the mission
solves a real need, has a reasonably well-defined technical design, and is
likely to succeed, and two independent planning reviews from different model
families check that before the human sees the brief, which shows the mission's
sizing and derisking at the top. An orchestrator completes an approved mission
or aborts it back to planning, with no human work inside it; human work after
it is a loose end. An implementer owns its item's code and review loop and
escalates to the orchestrator (atelier-ss4y).

# Success Criteria

- planner-end-to-end: a fresh planner agent given only the new skill replans
  mission atelier-yfqu from Sumeet's recorded direction (its 2026-11-08 note).
  Its plan has a technical design, sizing with a worth-it sentence, a derisking
  plan whose riskiest item comes first, no human work inside the mission, and
  two recorded planning review passes from two model families other than the
  planner's; `atelier brief atelier-yfqu` shows the sizing and derisking at the
  top. Shown by validation item atelier-v3nm, read by hand; Sumeet's approval of the site
  plan is the site mission's own gate, not this criterion.

# Sizing

- Complexity: medium. It restructures three roles across the skill's
  `SKILL.md`, `plan.md`, `orchestrate.md`, `implement.md`, `review.md`,
  `references/atelier.md` and `maintainers/skill-evaluation.md` (the last three
  through atelier-ss4y), adds two eval scenarios, and makes one small change to
  `atelier brief`; no new state.
- Value: high. Every mission's planning quality, and how often the human is
  pulled in, depend on it; atelier-yfqu waits on it.
- Risk: medium. The published skill steers every agent in every Atelier
  repository; a bad rule shows in the next mission and is undone by a revert.
- Worth it: the cheaper alternative, adding design and sizing lines to
  `plan.md` without a planner, leaves planning unowned, which is why those
  steps were missing.

# Shape

- **Roles and who decides.** The planner owns the plan until the human
  approves it, then hands the mission to an orchestrator. The orchestrator owns
  the unattended run: "Complete the mission. Abort only when its outcome can't
  be reached within the strategy, or carrying on would do harm"; aborting means
  stopping and reporting what was learned, and the human replans with a
  planner. It still plans a later increment at its evidence boundary within
  the approved strategy and Authority, with one independent planning review
  and no brief to the human; only a change to the strategy goes back to a
  planner and the human (Sumeet, 2026-11-08). The implementer owns its
  code and review loop and escalates to the orchestrator (atelier-ss4y keeps
  its reviewed plan).
- **Design, as files.**
  - `CONTEXT.md`: Mission says why it exists (two human reviews); Review names
    plan review and result review.
  - `procedures/plan.md` becomes the planner's procedure: the directive,
    grooming the area, sizing on `references/sizing.md`'s scales with a
    worth-it sentence, a technical design in the strategy's new **Design**
    section (components and where code lives, data flow, interfaces,
    dependencies and why, and every choice costly to reverse), derisking (the
    riskiest
    assumption, how it is tested early, items ordered riskiest first), the
    readiness test, two planning review passes, the brief and handoff.
  - The strategy file gains **Sizing**, **Design** and **Derisking** sections.
    The design lives in the strategy, which goes on the base as planning and is
    reviewed with it; the items that build a component bring
    `docs/architecture/` up to date. `atelier brief` shows Sizing and Derisking
    under the outcome.
  - `procedures/orchestrate.md`: the complete-or-abort rule replaces "Stop and
    ask only at..."; a strategy problem aborts instead of pausing for the
    human.
  - `SKILL.md` lists `plan` as the planner's role; `references/atelier.md`'s
    Planning on the Base asks for two passes.
- **Two planning review passes.** Every mission plan gets two independent
  reviews before the human's brief, from model families other than the
  planner's and different from each other when available, run in parallel,
  neither seeing the other's findings. A fix is re-reviewed by either. With one
  family available, both passes still run, as fresh agents: Sumeet asked for
  different families "ideally", not always.
- **No read-path limits.** Sumeet dropped atelier-qj9d's line limits for this
  mission (2026-11-08): counts are guidance, and items report them without a
  target.
- When qualities conflict: plain instructions beat completeness, and fewer
  rules beat more.

# Derisking

- Riskiest assumption: that one planner procedure can hold design, sizing,
  derisking and review guidance and still read as plain instructions. Tested
  first: the planner item is the first to merge, its review
  judges readability, and the success criterion runs it on a real replan.
- Next: that "complete or abort" does not make orchestrators push through a
  hazard or abort over what they could settle. atelier-xc5y tests both with
  fresh agents against the combined skill after atelier-ss4y merges, and
  atelier-ss4y's own evaluation covers the implementer's escalation.
- Two model families for planning review depended on Grok being startable as a
  reviewer here; this mission's own second planning pass ran on Grok 4.6, so it
  is settled.

# Scope

In: `CONTEXT.md`, the skill's `SKILL.md`, `plan.md`, `orchestrate.md`,
`references/atelier.md`, `atelier brief`'s top section and its product doc
(`docs/product/mission-reviews.md`), atelier-ss4y with its child atelier-ixdi,
the orchestrator evals (atelier-xc5y), and the validation replan of
atelier-yfqu.

Out: building the site; moving brief and debrief onto it (the site mission);
dropping `--html` (the site mission, once its pages exist); a CLI check for
missing Sizing (the brief shows it missing, which is enough); the over-correction
evals (atelier-jxce).

# Authority

The orchestrator may reorder and split items and choose wording within this
Shape. Changing the outcome, the two-pass rule or the roles needs Sumeet.

# Risks and Unknowns

- atelier-ss4y edits `orchestrate.md` after atelier-n7c4, and `SKILL.md` and
  `references/atelier.md` after atelier-xb18; it waits on atelier-n7c4, so
  they merge in sequence and the later one merges the base in.
- The skill will grow (about 1,065 lines today). With the limits dropped, the
  risk is readability, which each item's review judges.
