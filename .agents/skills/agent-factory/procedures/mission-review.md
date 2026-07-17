# Mission Review

Use this subskill for independent, read-only readiness review of one exact
mission draft and its complete reachable issue graph. It attempts to falsify
whether the graph can safely enter execution. It is not the `plan` procedure,
code or artifact `review`, scenario-centered `validate`, implementation work,
or merge authority.

## Required Assignment Inputs

The assignment must name all of the following before review begins:

```text
Repository: <absolute path>
Mission ID: <mission-id>
Exact graph revision: <revision algorithm version and digest>
Complete graph scope: <mission record, direct advances roots, every reachable
  hierarchy descendant, and all workflow-driving blockers/dependencies,
  including external prerequisite IDs>
Evidence destination: <mission-plan event, issue note, or first-class evidence
  target for findings or approval>
Independence context: <initial author identity and every material editor for
  this revision; assigned reviewer identity>
```

Do not construct a revision from partial records, infer missing graph content,
or substitute a branch commit, a mutable status, or a code-review approval for
the supplied exact graph revision. If a required input, attributable author or
material-editor identity, or complete graph projection is absent or untrusted,
report a blocking provenance/scope finding. Do not approve.

## Stance

- Be independent and read-only. Do not edit the mission, issues, links, or
  authored planning prose; do not implement repairs. Return required repairs to
  a separately assigned planner or material editor.
- Verify that the reviewer actor is distinct from the mission author and each
  actor whose attributed material change contributes to the exact revision.
  Display names, provider bots, or role labels do not establish independence.
- In Atelier repositories, use `atelier man reviewer`, `atelier status`,
  focused mission/issue views, and the repository's review-event surfaces for
  tactical commands. Read the current authoring standard, ADR, and validation
  guidance before making a judgment.
- Treat the mission's direct `advances` roots plus every unique canonical
  hierarchy descendant as the reachable work scope. Inspect canonical hierarchy
  and `advances` edges, workflow-driving internal blocker/dependency edges,
  and declared external prerequisite IDs. Context-only `relates` links do not
  supply scope or readiness.
- Bind every finding or approval to the supplied revision. A later material
  edit makes that result stale. Status, notes, activity, evidence receipts,
  timestamps, code commits, cache state, and context-only links do not by
  themselves replace the exact graph revision under review.

## Falsification Rubric

Apply the repository's normative mission authoring standard. Attempt to find a
blocking defect in each of these areas:

1. **Outcome, non-scope, and coverage.** The mission and each accountable item
   have a worker-usable Outcome. Every material mission claim maps to reachable
   work, and every reachable item maps back to a claim or explicit integration,
   migration, documentation, cleanup, or closeout need.
2. **Decomposition and accountability.** Epics are coherent integration/review
   boundaries; slices have one observable owner; cross-slice interfaces and
   final integration have an explicit accountable issue. Reject duplicate,
   implied, or unowned work.
3. **Dependencies and sequencing.** Follow dependency paths from every
   initially runnable issue. Internal blockers must be declared and acyclic;
   declared external prerequisites must name a stable external ID or durable
   artifact, its owner, and the consumer it blocks. An open or unowned
   transitive prerequisite defeats readiness.
4. **Unresolved decisions.** Architecture, product, security, migration, or
   public-contract decisions that can change downstream work have an explicit
   decision/artifact owner and block their consumers.
5. **Validation and closeout.** Required independent validation, review,
   migration, documentation, obsolete-path cleanup, and parent closeout each
   have accountable work. Validation derives later scenarios from Outcomes; do
   not require planner-authored transcripts or treat this review as outcome
   proof.
6. **Initial parallelism.** Initially runnable items have disjoint write
   ownership, or an explicit synchronization and integration plan. Flag races
   over canonical files, schemas, interface decisions, or release/merge
   actions.
7. **Graph legibility and freshness.** Roots, hierarchy, scope edges,
   blockers, validation links, provenance, and revision are sufficient to make
   the judgment without private history. Flag duplicate roots without a stated
   reason, unreachable required work, cycles, stale review events, or missing
   material-edit attribution.

## Output

Record one of the following against the exact graph revision in the assignment
destination:

```text
Mission: <mission-id>
Graph revision: <algorithm-version:digest>
Reviewer: <stable actor identity>
Authors/material editors checked: <stable actor identities>

Findings
- MR-001 [blocking|non-blocking] Criterion: <rubric item>.
  Affected: <stable issue ID(s)>.
  Path: <full dependency path, when readiness/dependency-related>.
  Falsification: <what record or edge contradicts readiness>.
  Required repair: <planner/material editor action>.
```

If and only if no blocking finding remains, provenance and independence are
complete, and the exact revision is current, record an approval that says:

```text
Approved mission: <mission-id>
Approved graph revision: <algorithm-version:digest>
Reviewer: <stable actor identity, independent of author and every material editor>
Scope inspected: <roots, reachable issue IDs, and external prerequisite IDs>
```

Approval permits only the repository-configured mission planning transition. It
does not approve a code changeset, authorize merge, prove implementation
outcomes, close validation, or let the reviewer repair the authored graph.

## Handoff

Report the mission ID, exact revision, complete scope inspected, stable actor
identities checked for independence, findings or approval event/evidence ID,
commands run, uninspected inputs, and the exact planner or orchestrator
follow-up. If findings require repair, stop after the read-only report.
