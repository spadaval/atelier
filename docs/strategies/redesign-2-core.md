---
status: draft
last_changed: "2026-11-09: first draft"
---

# Why

Agents cannot be oriented today because the tool does not know who is asking
or what stage the work is in. They find their place by running a command and
reading the refusal: 38 of 50 sampled refusals were friction, agents ran 541
help lookups in five days, and orchestrators spent 48% of delegated spend on
coordination. The [product specification](redesign-product.md) replaces the
47 commands and eight issue types with 17 commands over missions, items and
reviews, each bound to a thread and a stored status.

# Outcome

A new `atelier` binary on master implements the product specification's
records and commands. It is not installed anywhere. A fresh agent, told only
to run `atelier status`, takes a ready item to done with an independent
reviewer. A converted scratch clone of each of the five repositories passes
`check` and completes one item. The old commands, their code and their tests
are gone from the tree.

# Success Criteria

- fresh-agent-loop: in the eval harness, a fresh Claude Opus agent and a
  fresh Codex agent, each given a fixture repository with one ready item and
  the instruction "run `atelier status`", take the item to done with a
  reviewer child. Three runs each. No run has more than one refused or
  errored `atelier` call.
- rehearsal: the converter and the new binary, run on a scratch clone of each
  of the five repositories, pass `check`, and one item in each clone goes
  from ready to done.

# Sizing

- Complexity: high. It replaces the record layout and the whole command
  layer.
- Value: high. Every later mission and the scorecard depend on it.
- Risk: medium. Nothing installed changes during the mission. The last item
  removes the old tool from master, and reverting it restores it.

It is worth its cost against the cheaper alternative, fixing fumbles one at a
time: about 75 such fixes have merged since October, and the refusal rate in
the product repositories rose from 2.4% to 5.1% over the same weeks. Dropping
the tool for a skill alone would lose the stored status, the thread binding
and the records, which a skill cannot provide.

# Shape

- **The loop is rebuilt as one unit.** The record layout changes under every
  command at once, so commands cannot be replaced one at a time. What stays
  in place is the library code below the commands: Git and shell adapters,
  the forge and CI code, the merged-tree checks, Markdown and YAML parsing,
  ID generation and the site.
- **New beside old, then one swap.** New code goes into the existing crates
  as new modules and is reached through a second binary target,
  `atelier-next`, in `atelier-cli`. The old binary, its tests, the CI `check`
  step and `scripts/smoke-repos.sh` keep working untouched until the swap.
  The last item deletes the old modules and tests and renames the binary.
  The tree is larger until then and smaller after.
- **The installed 0.4.0 tracks this mission's own work.** The repository's
  records stay in the old layout until mission 3 converts them.
- **The specifications are the scope.** When qualities conflict: correctness
  of the lifecycle first, then how little an agent must read to act, then
  code size.

# Design

The design is the [architecture specification](redesign-architecture.md). It
fixes the crate layout, the record formats, the act table, thread identity,
where records are read and written, and the merge steps. This mission builds
it in six steps:

| Step | Builds | Ready now |
| --- | --- | --- |
| A. Skeleton | Item, review and config records; thread identity; the act table; `start`, `submit`, `review`, `verdict`, `merge`, `status`; the fresh-agent scenario | Yes |
| B. Planning | Mission records; `new`, `set`, `ready`, `drop`, `close`, `list`, `show`, `lead` | No |
| C. Disagreement | `escalate`, `decide`, rebuttals in the brief, `list --deferred` | No |
| D. Handoff | `land`, `check`, `setup codex`, the hooks' output | No |
| E. Site | The site reads the new layout and shows what changed from the plan | No |
| F. Swap | The converter, the rehearsal, deleting the old code and tests, renaming the binary, pinning CI's `check` to 0.4.0 | No |

Steps B to F wait in the backlog with preliminary goals. Each is planned at
its boundary, with one planning review, once the step before it has shown
what it shows.

Each item that builds a component brings the architecture specification up to
date with what was built.

# Derisking

The riskiest assumption is that an agent shown its status and next command
will follow them without fumbling. Everything else in the redesign rests on
it. Step A tests it first and cheaply: it builds only the six commands of
the work loop on a fixture repository, then runs the fresh-agent scenario
with Claude Opus and with Codex before anything else is built. If agents
still fumble, the fix is in the wording and shape of `status`, and that is
cheapest to change while only six commands exist.

The second risk is the swap. The rehearsal in step F runs the converter and
the new binary on scratch clones of all five repositories before any old code
is deleted.

# Scope

- In: the records, commands, status model, thread binding, merge, config,
  site reading and converter in the two specifications.
- Out: installing the binary; converting any real repository; the Agent
  Factory skill; the gardener and scorecard; a session database; live thread
  status; any command, record kind, status or config option the
  specifications do not name.
- Target environment: Linux, Git 2.40 or later, T3 Code, with and without a
  GitHub forge.

# Authority

The orchestrator may decide flag names, output wording, module names, test
structure, and the order of work inside a step. It plans steps B to F at
their boundaries. It may cut anything in the specifications that proves
unnecessary, and reports each cut.

It may not add a command, record kind, status, config option or dependency
the specifications do not name, or change the product specification's Words,
Lifecycle or Review sections. Those go to Sumeet.

**Scope rule.** A review finding that asks for behaviour the specifications
do not name is not fixed in place. The implementer escalates it, and the
orchestrator defers it or, if it shows the specification is wrong, takes it
to Sumeet.

# Risks and Unknowns

- **`status` wording.** The first wording will be wrong in places. Step A's
  scenario runs are where it is found.
- **Reading started items from their branches** on every `status` and `lead`
  may be slow in astromech, which has the most branches. The site already
  does this read. If it is slow, the fix is a cache inside one command's run,
  not a stored index.
- **The Codex session-start hook event** is not yet known. Step D finds it.
  If none fits, `AGENTS.md` tells Codex to run `atelier status` first.
- **Tree size rises during the mission.** Both command layers exist until
  the swap. The report gives the line count before and after it.
