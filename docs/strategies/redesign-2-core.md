---
status: draft
last_changed: "2026-11-09: amended at Sumeet's direction: the tool is rewritten in full, in a fresh workspace"
---

# Why

Agents cannot be oriented today because the tool does not know who is asking
or what stage the work is in. They find their place by running a command and
reading the refusal: 38 of 50 sampled refusals were friction, agents ran 541
help lookups in five days, and orchestrators spent 48% of delegated spend on
coordination. The [product specification](redesign-product.md) replaces the
47 commands and eight issue types with 18 commands over missions, items and
reviews, each bound to a thread and a stored status.

# Outcome

A new workspace on master, `next/`, holds the tool rewritten in full: a new
`atelier` binary that implements the product specification's records and
commands. It is not installed anywhere, and the old workspace beside it is
untouched. A fresh agent, told only to run
`atelier status`, takes a ready item to done with an independent reviewer. A
converted scratch clone of each of the five repositories passes `check`, and
one item completes in two of them.

# Success Criteria

- fresh-agent-loop: in the eval harness, a fresh Claude Opus agent and a
  fresh Codex agent, each given a fixture repository with one ready item and
  the instruction "run `atelier status`", take the item to done with a
  reviewer child. Three runs each. No run has more than one refused or
  errored `atelier` call.
- rehearsal: the converter and the new binary, run on a scratch clone of each
  of the five repositories, pass `--verify` and `check`. One item goes from
  ready to done in the atelier clone and in one product clone.

# Sizing

- Complexity: high. It writes the tool again, in full.
- Value: high. Every later mission and the scorecard depend on it.
- Risk: low. Nothing installed changes, and no file of the old workspace is
  edited.

It is worth its cost against the cheaper alternative, fixing fumbles one at a
time: about 75 such fixes have merged since October, and the refusal rate in
the product repositories rose from 2.4% to 5.1% over the same weeks. Dropping
the tool for a skill alone would lose the stored status, the thread binding
and the records, which a skill cannot provide.

# Shape

- **A full rewrite, in a fresh workspace.** The tool is written again in
  `next/`, a Cargo workspace of its own that depends on nothing under
  `crates/`. The record layout changes under every command at once, and what
  could have stayed in place is bound to types that go away.
- **Copy on purpose, write the rest.** A piece of old code is copied only
  when its behaviour is wanted as it is, the item that copies it names it,
  and it is reviewed as new code. The architecture specification lists the
  pieces: the forge code; the Git calls the commands use and the reader of
  files at a commit; the merged-tree checkout with the lock and the
  re-check that protect a merge from a base or branch that moved; the
  front-matter splitter, changed to keep a body byte for byte; ID
  generation; and the merging of hook settings.
- **The old workspace is not edited.** Its binary, its tests, the CI `check`
  step and `scripts/smoke-repos.sh` keep working. Deleting it, rebuilding
  the site and switching belong to mission 3. The tree is larger until then
  and much smaller after.
- **The installed 0.4.0 tracks this mission's own work.** The repository's
  records stay in the old layout until mission 3 converts them.
- **The specifications are the scope.** When qualities conflict: correctness
  of the lifecycle first, then how little an agent must read to act, then
  code size.

# Design

The design is the [architecture specification](redesign-architecture.md). It
fixes the crate layout, the record formats, the act table, thread identity,
where records are read and written, and the merge steps. This mission builds
it in five steps:

| Step | Builds | Ready now |
| --- | --- | --- |
| A. Skeleton | Item, review and config records; thread identity; the act table; finding started items by branch; `start` with take-over, `submit`, `review` with rounds, `verdict`, `merge`, `status`; the fresh-agent scenario | Yes |
| B. Planning | Mission records; `new`, `set`, `ready`, `drop`, `close`, `list`, `show`, `lead`, `start --new`; plan reviews | No |
| C. Disagreement | `escalate`, `decide` with its two outcomes, rebuttals and decisions in the brief, `list --deferred`, and settling a deferred finding with `new --from` and `drop` | No |
| D. Guards | `check`, the three Git hooks, `ATELIER_SKIP`, and `setup` for Git and for Codex | No |
| E. Rehearsal | The converter, with `config.toml` written from `workflow.yaml` and a `--verify` mode, and the rehearsal on a scratch clone of each of the five repositories | No |

Steps B to E wait in the backlog with preliminary goals. Each is planned at
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

The second risk is that started work is found by its branch and not by a
stored field. A wrong answer there would have `status` hand out work another
thread already holds. Step A's scenario tests start one item from two
checkouts at once, interrupt a start, take an item over in every started
status, and read a started item from the base.

The third is real data. The rehearsal in step E runs the converter and the
new binary on scratch clones of all five repositories.

# Scope

- In: the records, commands, status model, thread binding, merge, config and
  converter in the two specifications.
- Out: `log`, which mission 4 builds, so this mission builds 17 of the 18
  commands; installing the binary; converting any real repository; the site;
  editing or deleting anything under `crates/`, which mission 3 does whole;
  coordinating two clones of one repository; the
  Agent Factory skill; the gardener and scorecard; a session database; live
  thread status; any command, record kind, status or config option the
  specifications do not name.
- Target environment: Linux, Git 2.40 or later, T3 Code, with and without a
  GitHub forge.

# Authority

The orchestrator may decide flag names, output wording, module names, test
structure, and the order of work inside a step. It plans steps B to E at
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
- **Tree size rises during the mission.** Both workspaces exist until mission
  3's switch. The report gives the new workspace's line count at the
  mission's end, against the old one's 48,988.
- **A third rebuild.** The architecture specification says why this one is
  full and what makes it safe: fixed specifications, the old tool tracking
  the work throughout, a rehearsal on every repository, and a baseline to
  judge it against.
- **Something the old code handled is forgotten.** The copied pieces carry
  their contract tests. For the rest, the scenario tests and the rehearsal
  on five real repositories are where a missing case shows.
