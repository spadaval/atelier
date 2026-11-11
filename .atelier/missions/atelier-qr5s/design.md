---
status: draft
last_changed: "2026-11-11: amended after mission 2 closed: what it showed is listed, the orchestrator plans each step at its boundary, and the switch waits for Sumeet to name the time"
---

# Sizing

- Complexity: medium. The steps are known; the skill rewrite and its evals
  are the large parts.
- Value: high. Nothing in the redesign helps until it is in use.
- Risk: medium. A bad cutover stops work in five repositories, shows at once,
  and has a rehearsed undo for each place it can stop.

It is worth its cost because a redesign that is built and not adopted is the
June system again: 31% of the Rust ever written here survives.

# Shape

- **Prepared by ordinary items, run from a runbook.** Everything that needs
  judgment is reviewed and merged under the old tool before the cutover: the
  skill, the switch of the binary, the new `AGENTS.md` text and
  the replacement CI workflows. The
  cutover itself only runs a reviewed script, so it needs no review while it
  runs and neither tool has to gate a change it cannot read.
- **Judgment in the skill, procedure in the tool.** A role's file says how to
  decide. It does not repeat what `status` and `--help` print. The roles are
  lead, implementer, reviewer, decider and planner.
- **A procedure stays only if an eval shows it matters.** A scenario where
  the skill and no-skill arms do not differ is evidence to cut the text, not
  to keep it.
- **Independent review and the three exits of a fix-now finding are fixed.**
  They are Sumeet's, and the evals do not put them up for removal.
- **The window is short, and Sumeet opens it.** Once the switch merges, the
  tree's binary cannot read this repository's records, so the cutover runs
  straight after it. Everything up to the switch's approval is prepared
  without him. The switch merges only once he has named the time.
- **It waits on the baseline's frozen inputs, not on the baseline report.**

# Design

Nine steps. Each waits in the backlog with a preliminary goal and is planned
in full at its boundary, with one planning review by a fresh agent from
another lab before its work starts. Steps F and G get a second planning
reviewer, because they are the two that cannot be quietly undone. Steps A, C
and D depend on nothing in this mission and start together. The protocol is
the Transition section of the
[architecture specification](../../../docs/strategies/redesign-architecture.md#transition).

| Step | Does |
| --- | --- |
| A. Skill | Write the new skill in `skills/agent-factory-next/`, beside the live one: `SKILL.md` and one file per role against the new commands. Leave out what the tool now prints. Carry the constitution's live rules into the role files. The cutover moves it into place. |
| B. Evals | Port the scenarios to the new binary. Add a no-skill arm and a Codex arm. Cut each procedure no scenario supports. |
| C. Backlog sort | Sort the 147 open items and five open missions: converted as they are, rewritten for the new tool, or dropped as obsolete, with the list in the report. |
| D. Fallback release | Fix the old tool's fallback release: 0.4.0 if it checks this repository's records and serves the site, otherwise a new 0.4.1. |
| E. Site | Take the tracker's site out: the `atelier` binary built from `next/` has no site crate and no `serve`, its build and release need no Node, and the old site's web jobs leave CI. The Desk's own crate, app and CI jobs are not touched. The old site goes with the old workspace at the switch. The Desk replaces it in its own mission (atelier-z4cr, [ADR 0032](../../../docs/adr/0032-the-desk-is-the-humans-product.md)), which this mission does not wait for. |
| F. Switch | Delete the old workspace, `crates/`, with all its tests, and move the new one from `next/` to the repository root. Add the runbook, the new `AGENTS.md` texts, the replacement CI workflows and CI's two-way `check` step. Try the rollback on scratch clones at each stopping point. |
| G. Cutover | Run the runbook on this host. The item is never started; it is closed on the base at the end. |
| H. First work | Run one item in atelier and one in a product repository. Fix what first use breaks. Remove the fallback branch of CI's `check` step. |
| I. Docs | Replace `docs/product/`, `CONTEXT.md`, `PRODUCT_INTENT.md` and the ADRs with the design record and the current architecture pages. Move open missions' strategies into their directories. |

The skill's size is reported before and after as an indication. The current
agent-facing text is 1,219 lines.

## What mission 2 showed

Mission 2 (atelier-fcb7) closed on 2026-11-11 with both criteria proved. Its
dated notes hold the detail. The steps below plan against these facts:

- **Step A.** Fresh agents followed `status` with no skill at all, so the
  skill starts from nothing and adds only what a scenario shows is missing.
  The binary leaves three Review rules to the skill: a rebuttal's one try,
  escalation after three rounds, and a fresh reviewer for a rebuttal. A
  harness's own subagent has its parent's thread identity and is refused as
  a reviewer, so the skill says to start reviewers and deciders as threads.
  Two items change the live skill in ways the new one must also meet:
  atelier-8ep4 (each role gets only its own instructions) and atelier-y0qe
  (which roles work with the human). Step A's plan says, for each, whether
  the new skill starts from its merged text or absorbs it.
- **Step B.** No fresh agent has met a rebuttal, `escalate`, `decide`, a
  third round, or a refusal that offers `ATELIER_SKIP` (atelier-42b4). Those
  are where a skill is most likely to matter, so the ported scenarios cover
  them.
- **Step C.** 58 ready items across the five repositories belong to a draft
  mission and cannot start under the new tool. 93 ready items have a Done
  when with no checkbox, which the new tool counts as ticked (atelier-t6o6,
  point 4).
- **Step F.** The pre-push hook refuses the conversion push unless
  `ATELIER_SKIP` is set or the push comes before `setup`. No product
  repository configures a forge, so after conversion nothing pushes their
  base unless the conversion writes a `[forge]` section or their new
  `AGENTS.md` says how. astromech's CI installs Atelier and runs `doctor`
  and `land`, which no longer exist. 11 tracked files outside `.atelier/`
  name a moved strategy file's old path; the converter lists them. One
  repository's checkout sits on a feature branch, so the runbook clones and
  converts on the base branch. `setup codex` writes a `SessionStart` hook
  that Codex runs only after the user trusts it.
- **Step G.** The proof of each real conversion is a `--verify` pass and the
  count of relative links that still resolve. The new binary has not yet
  talked to real GitHub, been built for release, or run its Claude Code
  hooks in a live session; step F's plan says where each first happens
  before the cutover depends on it.

Six points mission 2 left to Sumeet are in atelier-t6o6. Steps A to E do not
wait for them: the skill carries the Review section's rules as they stand.
Points 1, 4 and 5 are settled before step F's plan is reviewed, since the
converter, the skill and the runbook each read one of them.

# Derisking

The riskiest assumption is that the cutover leaves no repository stuck.
Mission 2's rehearsal converts scratch clones of all five and runs an item in
two. Step F tries the rollback on scratch clones at each stopping point the
architecture specification lists, including a conversion that fails before
its commit and one that is committed and not pushed.

The second is that the evals can tell the arms apart at three runs per cell.
If a scenario cannot, its procedure is cut. That is the cheap direction to be
wrong in: text can be added back when a real failure shows the need.

# Risks and Unknowns

- **No brief or debrief page for a while.** From the cutover until the
  Desk's mission page is live, Sumeet reads a mission with `atelier show
  <mission>` and its files. Step H's first work runs in that gap.

- **Work in flight at the cutover.** A branch not merged or abandoned before
  conversion loses its reviews. The runbook's first step stops while any
  item is in progress.
- **Sumeet's other threads.** An agent working in a product repository during
  the cutover would meet a tool that changed under it. The runbook checks
  T3's running threads first and stops.
- **A reinstall between the switch and the cutover** would install a binary
  that cannot read any repository's records. `AGENTS.md` says not to, and
  the cutover follows the switch at once.
- **A repository whose forge requires a pull request for its base.** The
  rehearsal finds out, and that repository's conversion then goes as one.
- **The evals may show the skill adds little.** Then the skill gets small.
  That is an acceptable result.
- **The work host lags.** Until Sumeet converts it, it runs the old binary,
  the old skill and old records, which still work together. It must not
  fetch a converted repository before then.
