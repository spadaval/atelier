---
status: draft
last_changed: "2026-11-09: revised after planning review round 1: the cutover protocol, waits on the freeze not the report"
---

# Why

Mission 2 leaves a second binary on master that nothing uses. The installed
binary and the installed Agent Factory skill are each one per host, so every
repository on a host changes process together. This mission makes that change
once, with a rehearsal behind it and a way back.

The skill has to change with the tool. It names commands that will no longer
exist, and its own evals have never shown it beating no skill: the one
comparison found no gain and was contaminated, and no clean no-skill arm has
run since.

# Outcome

On this host, atelier, astromech, demiurge, life-on-rails and cosplay-studio
run on the new records and the new binary. Agent Factory is rewritten against
the new tool, and each procedure it keeps has an eval scenario it changed.
The old command layer and the library code only it used are deleted. The old
docs and the ADRs are replaced by the design record. Sumeet has a runbook to
convert his work host.

# Success Criteria

- five-converted: with the new binary installed, `atelier check` passes in
  all five repositories. The cutover item itself went from building to done
  under the new binary in atelier, and one item went from ready to done in a
  product repository.
- no-old-surface: a search for each removed command and record path finds
  nothing in the skill, in the five repositories' `AGENTS.md` and
  `CLAUDE.md`, or under `docs/`.
- skill-earns-its-place: a results file records skill against no skill, on
  Claude Opus and on one Codex model, at least three runs per cell. Every
  procedure the skill keeps names a scenario where it changed the outcome.

# Sizing

- Complexity: medium. The steps are known; the skill rewrite and its evals
  are the large part.
- Value: high. Nothing in the redesign helps until it is in use.
- Risk: medium. A bad cutover stops work in five repositories, shows at once,
  and is undone by reinstalling 0.4.0 and reverting one commit in each.

It is worth its cost because a redesign that is built and not adopted is the
June system again: 31% of the Rust ever written here survives.

# Shape

- **The skill comes first.** The cutover needs it, and its eval runs on the
  new binary are the last rehearsal.
- **Judgment in the skill, procedure in the tool.** A role's file says how to
  decide. It does not repeat what `status` and `--help` print. The roles are
  lead, implementer, reviewer, decider and planner.
- **A procedure stays only if an eval shows it matters.** A scenario where
  the skill and no-skill arms do not differ is evidence to cut the text, not
  to keep it.
- **Independent review and the three exits of a fix-now finding are fixed.**
  They are Sumeet's, and the evals do not put them up for removal.
- **One cutover, atelier first.** All five convert in one sitting. The
  cutover is one item that starts under the old tool and finishes under the
  new one: the converter converts it in flight, and its own review and merge
  are the first use of the new loop.
- **It waits on the baseline's frozen inputs, not on the baseline report.**

# Design

Six steps, all in the backlog until mission 2's evidence is in.

| Step | Does |
| --- | --- |
| A. Skill | Rewrite `SKILL.md` and one file per role against the new commands. Move what the tool now prints out of the skill. Fold the constitution's live rules into the role files and delete the rest. It merges to master and does not go live until the cutover moves the skill checkout. |
| B. Evals | Port the scenarios to `atelier-next`. Add a no-skill arm and a Codex arm. Cut each procedure no scenario supports. |
| C. Backlog sort | Sort the 147 open items and five open missions: converted as they are, rewritten for the new tool, or dropped as obsolete, with the list in the report. |
| D. Cutover | The one item the [architecture specification](redesign-architecture.md#transition) describes: convert atelier and switch the binary on its branch, review and merge it with the new binary, install, then convert the other four. The rollback is tried on scratch clones first. |
| E. Clean-up and first work | Delete the library code nothing reaches any more. Run one item in a product repository. Fix what first use breaks. |
| F. Docs | Replace `docs/product/`, `CONTEXT.md`, `PRODUCT_INTENT.md` and the ADRs with the design record and the current architecture pages. Move open missions' strategies into their directories. |

The skill's size is reported before and after as an indication. The current
agent-facing text is 1,219 lines.

# Derisking

The riskiest assumption is that the cutover leaves no repository stuck.
Mission 2's rehearsal converts scratch clones of all five and runs an item in
each. This mission adds a rollback that is tried once on a scratch clone
before the real cutover: reinstall the 0.4.0 release, revert the conversion
commit, and run the old `atelier status`.

The second is that the evals can tell the arms apart at three runs per cell.
If a scenario cannot, its procedure is cut. That is the cheap direction to be
wrong in: text can be added back when a real failure shows the need.

# Scope

- In: the skill, its evals, the backlog sort, the cutover on this host, first
  use, and the docs.
- Out: the work host, which Sumeet converts with the runbook; the gardener,
  groom and audit procedures, which mission 4 rewrites and which are carried
  over unchanged except for command names; new scenarios beyond those needed
  to judge the kept procedures.
- Target environment: this host; Claude Opus and a Codex model for the evals.

# Authority

The orchestrator may choose the cutover time, once no work is in flight in
any of the five repositories and no thread is running in them. It may cut any
skill procedure no scenario supports, drop the backlog items the sort finds
obsolete, and fix what first use breaks inside the specifications.

It may not weaken the independent review rule or the three exits of a fix-now
finding, change the specifications' Words, Lifecycle or Review sections, or
touch the work host.

**Scope rule.** As in mission 2: a finding that asks for behaviour the
specifications do not name is escalated, not fixed in place.

# Risks and Unknowns

- **Work in flight at the cutover.** A branch not merged or abandoned before
  conversion loses its reviews. The runbook's first step refuses to go on
  while any item but the cutover item is in progress.
- **The cutover item's own review is large.** It holds the converted records
  and the deletion of the old command layer and its tests. Both are
  mechanical: the reviewer checks the conversion against the rehearsal's
  output and that the code change only deletes and re-points.
- **Sumeet's other threads.** An agent working in a product repository during
  the cutover would meet a tool that changed under it. The orchestrator
  checks T3's running threads first and waits.
- **The evals may show the skill adds little.** Then the skill gets small.
  That is an acceptable result.
- **The work host lags.** Until Sumeet converts it, it runs the old binary,
  the old skill and old records, which still work together.
