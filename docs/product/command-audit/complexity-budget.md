# Command Complexity Budget

Atelier commands now have a complexity budget. A command survives only when the
operator job is real, frequent or high-risk enough to justify a separate public
surface.

This budget follows the Zen principle: choose the simplest thing that works.
Prefer the least magical solution that solves the real problem.

Complexity has two budgets:

- Product complexity: the cognitive work an operator must do to choose the
  command, parse the output, decide which facts matter, and stitch commands
  together before acting.
- Architecture complexity: the code, queries, caching, renderers, tests,
  migrations, and ownership boundaries required to support the behavior.

A command is healthy when it lowers product complexity enough to justify its
architecture cost. A shared generic command is not automatically simpler if it
forces operators to scan a firehose, remember flags, or compose several
commands. A purpose-built view is not automatically better if it duplicates
read models, forks lifecycle interpretation, or needs special formatting tricks
to feel coherent.

## Decision Record Contract

This is a decision aid, not a workflow or an additional approval gate. For an
audit entry, record only the operator question, role, product/cognitive cost,
architecture/code cost, one `Keep`, `Simplify`, `Fold`, `Hide`, or `Remove`
verdict, and the next action. Use short evidence-backed phrases; do not turn a
command audit into a narrative or a separate process.

## Budget Test

For every command or flag family, ask:

- What operator question does this answer?
- Which role asks that question during normal work?
- Can an existing command answer that question with less surface area?
- Does the command expose domain work, or implementation machinery?
- Does it require hidden inference, special cases, or formatter cleverness to
  feel usable?
- Does the default output show enough detail to understand state without
  dumping every available fact?
- Would deleting it remove a real capability, or only an alternate spelling?

Then ask the code-side questions:

- Does the command reuse an owned read model, or does it create another
  interpretation of workflow, blockers, evidence, or hierarchy?
- Does the renderer only render supplied facts, or does it query, infer,
  validate, or mutate?
- Does the feature need broad cache, migration, fixture, or compatibility work
  for a narrow operator benefit?
- Would a smaller view, fewer flags, or one shared read model reduce both code
  complexity and operator confusion?

If the answer is unclear, the command does not get the benefit of the doubt.
Hide it, fold it into the owner command, or remove it.

## Verdicts

- Keep: the command owns a distinct, valuable operator job.
- Simplify: the job is real, but the command or output exceeds its budget.
- Fold: the capability should move into an existing owner command.
- Hide: the command is useful only for admin, migration, recovery, or debug.
- Remove: the command preserves vocabulary, plumbing, or alternate paths with
  little product value.

## Current Budget Pressure

| Surface | Pressure | Target |
| --- | --- | --- |
| `issue list` | A simple generic inventory is now available; it must not grow into a dashboard. | Keep it inventory-shaped and route operational decisions to `work` views. |
| `work queue` | Being asked to cover generic inventory, mission discovery, top-level work selection, and operational queues. Its current repo-wide nested output does not clearly belong to one Agent Factory role. | Simplify or fold. Keep only if it has a crisp repo-wide operational job that reduces scanning and command stitching more than `work ready`, `work blocked`, `work active`, `work mission`, `work epic`, and `issue list` already do. |
| `work mission` | Current dashboard prints proof/closeout/ready-work noise before useful mission coordination. | Epic-first, mission-scoped, bounded, with drill-down flags. |
| `issue transition` | Default output leaks validators, action preflights, dirty-path dumps, descriptions, and commands with equal weight. | Default shows transitions and failed requirements only; verbose keeps full machinery. |
| `review` | Too many verbs mirror provider operations. | Collapse submit-like actions and infer issue/provider context where possible. |
| `evidence attach` | Separate verb for a typed cross-kind proof relationship. | Keep as the secondary reuse path; `record --target` remains the taught first step and generic issue links do not own evidence. |
| scoped `history` flags | Can become a second query language. | Keep only `--issue` and `--limit`; fold objective scope into `work mission`/`work epic` and remove query filters. |
| provider, branch, maintenance, diagnostics roots | Expose recovery or implementation machinery. | Hide unless explicitly needed for admin recovery. |

## Output Budget

Default human output should show the right layer first:

- state;
- failed requirements or blockers that affect the next action;
- bounded context;
- one drill-down path for detail.

Do not show passing validators, raw path firehoses, repeated command syntax,
provider plumbing, or full transcripts by default. Do not rewrite facts into
editorial prose when a direct list is clearer.
