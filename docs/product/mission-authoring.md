# Mission And Issue-Set Authoring Standard

This is the normative standard for authoring and independently reviewing a
mission's issue set. It applies before a mission is submitted for plan review.
It answers whether the reachable work graph is sufficient, decomposed,
sequenced, and reviewable; it does not prescribe code review, merge authority,
or the later execution evidence transcript.

The lifecycle, revision inputs, reviewer independence, findings, approval, and
migration rules remain owned by [ADR 0018](../adr/0018-independent-mission-plan-review.md).
[Work Model](work-model.md) owns record and relationship semantics, and
[Validation](../architecture/quality/validation.md) owns proof routing. Where
those sources disagree, ADR 0018 controls the review contract and the live
workflow controls executable transitions.

## What Is Being Authored

A mission is the outcome and scope boundary. Its scope is every unique issue
reachable from its direct `advances` roots through canonical hierarchy. An
authored graph therefore includes the mission, its reachable epics and issues,
its workflow-driving internal edges, and every declared external prerequisite
that can affect execution readiness.

Use the smallest accountable record that answers the question at hand:

| Record | Author it to own | Do not use it to own |
| --- | --- | --- |
| Mission | Target `Outcome`, explicit non-scope, risks, direct scope roots, and parent-level closeout expectations. | Child implementation steps or a prewritten execution transcript. |
| Epic | A coherent integration and branch/review boundary with a cohesive outcome. | An unbounded bucket of unrelated work or a duplicate mission spec. |
| Executable issue | One observable implementation, documentation, migration, cleanup, review, or artifact-update slice with one accountable owner. | A parent outcome, a hidden prerequisite, or another issue's work. |
| Validation issue | Independent judgment derived from an Outcome or explicit risk, including claim classification and evidence capture. | Repairing the work it is validating or copying the implementation plan. |
| Evidence record | A receipt for a check that ran: claim, result, actor, and inspectable command or artifact. | Work that still needs an accountable issue or approval of a plan. |

Use hierarchy for ownership, `advances` for mission scope/progress, and a
blocking/dependency edge only when one record must be satisfied before another
can proceed. A context link is never a substitute for scope or a blocker.

## Normative Authoring And Review Checklist

An author **must** make each item below inspectable in the records and links.
An independent reviewer **must** classify a missing, ambiguous, or contradicted
item as a finding; a blocking finding prevents approval until resolved.

1. **Outcome and non-scope.** The mission Outcome states the finished,
   observable repository or operator state, not a task list. It names material
   constraints and explicit non-scope. Each epic and executable issue has a
   local Outcome that says what its owner will make true.
2. **Complete outcome-to-work coverage.** Map every material mission Outcome
   claim to one or more reachable accountable issues, and map each reachable
   issue back to the claim it advances. A claim with no work is orphaned; work
   with no claim is scope drift unless it is explicitly necessary integration,
   migration, documentation, cleanup, or closeout work.
3. **Coherent decomposition and integration.** Split work at stable ownership,
   integration, or independent-validation boundaries. An epic must form one
   coherent reviewable change; its children must collectively deliver the epic
   outcome. Name the issue that integrates cross-slice changes, resolves shared
   interfaces, or owns the final coherent artifact. Do not leave integration
   implied by chat or by parallel branch names.
4. **Single accountability.** Every implementation, document, migration,
   cleanup, review, validation, and closeout result has exactly one accountable
   issue. Several issues may contribute to a claim, but two issues must not own
   the same result. Reframe shared work as a distinct integration issue or
   divide its observable boundaries.
5. **Dependencies, including external ones.** Declare each actual prerequisite
   with the blocking/dependency edge and identify its owner. A prerequisite
   outside the mission graph is still an explicit external dependency: name its
   stable ID or durable artifact and the consumer it blocks. Do not disguise an
   external dependency as non-binding prose or a context link.
6. **Transitive execution readiness.** Follow every dependency path from each
   initially runnable issue. No path may reach an open or unowned prerequisite.
   A direct blocker that is closed while its own external blocker remains open
   is not ready. Internal advancing work is execution scope, not automatically
   a blocker merely because it is unfinished.
7. **Unresolved decisions.** If an architecture, product, migration, security,
   or public-contract decision can change downstream work, create an explicit
   decision/artifact issue and block its consumers. Do not bury a decision in
   an implementation issue description or allow dependent work to guess.
8. **Validation and evidence.** Identify independent validation whenever the
   risk or contract requires it: parent completion, public behavior, workflow
   or Agent Factory policy, docs/help parity, migration, cross-cutting state,
   or a conflicted evaluator. Validation derives scenarios from Outcomes; the
   planner does not prewrite a future transcript. Every required validation and
   review result has an accountable issue, and its later proof is attached as
   first-class evidence.
9. **Migration, documentation, cleanup, and closeout.** When the Outcome
   changes a durable contract or replaces a path, include owned work for data or
   state migration, user/operator documentation, deletion or bounded cleanup
   of obsolete paths, and the parent closeout mapping. State why any category
   is not applicable. A parent is not closeable just because its main feature
   issue is done.
10. **Initial parallel safety.** The first concurrently runnable issues must
    have non-overlapping write ownership or an explicit synchronization and
    integration plan. They may share an Outcome, but must not race on the same
    canonical file, schema, interface decision, or release/merge action. Add a
    dependency or split a shared owner when they would.
11. **Reviewable graph.** Direct roots, hierarchy, `advances`, blockers, and
    validation links make scope and order readable without private history.
    The graph must contain no duplicate roots unless the deliberate reason is
    stated, no unreachable required work, and no cycle that makes readiness
    impossible. Material edits require the fresh review specified by ADR 0018.

## Fresh-Context Classification Examples

The examples use illustrative stable IDs. A reviewer who sees only these
records and edges can make the stated classification.

### Complete graph — approve if its record content matches

`atelier-orbit` has Outcome: “operators can request and inspect revision-bound
mission-plan review; the former direct-ready path is removed.” Its non-scope
excludes code-branch review changes. It directly advances the following roots:

```text
atelier-orbit (mission)
  advances atelier-contract (epic: event/revision contract)
    child atelier-digest (task: deterministic graph digest)
    child atelier-events (task: attributed review events)
    child atelier-contract-integration (task: integrate the public contract)
  advances atelier-lifecycle (epic: workflow transition enforcement)
    child atelier-ready-gate (task: require current approval)
    child atelier-migrate (task: explicit legacy-state migration)
  advances atelier-authoring (task: authoring and operator documentation)
  advances atelier-cleanup (task: remove the direct-ready path)
  advances atelier-validate (validation: independent end-to-end classification)

atelier-events blocks atelier-contract-integration
atelier-digest blocks atelier-contract-integration
atelier-contract-integration blocks atelier-ready-gate
atelier-ready-gate blocks atelier-validate
atelier-migrate blocks atelier-validate
atelier-authoring blocks atelier-validate
atelier-cleanup blocks atelier-validate
```

Classification: complete and initially parallel-safe. `atelier-digest`,
`atelier-events`, `atelier-migrate`, `atelier-authoring`, and
`atelier-cleanup` have disjoint ownership. `atelier-contract-integration` owns
the shared contract result; `atelier-validate` independently maps every mission
claim, migration, documentation, and removal result to evidence. The graph
shows both direct and transitive sequencing paths.

### Defective graph — reject with affected IDs and paths

| Finding | Affected ID(s) and path | Why it fails |
| --- | --- | --- |
| Missing external blocker | `atelier-ready-gate -> atelier-provider-contract` (external, open) | The issue assumes a provider API decision but only mentions it in Notes. The external artifact/issue must be declared as a prerequisite and own its decision before `atelier-ready-gate` is runnable. |
| Unsafe initial parallelism | `atelier-schema-v3` and `atelier-cache-rebuild` | Both are initially runnable and both edit the same canonical schema and rebuild rule. Add `atelier-schema-v3 -> atelier-cache-rebuild`, or make a named integration issue own the shared boundary. |
| Orphaned Outcome claim | `atelier-orbit`, claim “former direct-ready path is removed” | No reachable cleanup or migration issue maps to this claim. Add an accountable removal issue and its validation/closeout coverage. |
| Duplicate ownership | `atelier-help-docs` and `atelier-command-docs` | Both own the same `atelier issue transition` help wording. Give one issue the wording result and make the other own a distinct docs surface or depend on the first. |
| Malformed validation coverage | `atelier-validate -> atelier-ready-gate -> atelier-provider-contract` | `atelier-validate` says “tests pass” but does not independently classify the public lifecycle claim, the external contract path, migration, or docs/help parity. Its Outcome must derive those checks and record first-class evidence; it must not fix the defects itself. |

These are blocking authoring findings, not implementation failures. The author
repairs the graph, records the material edit, and requests a fresh independent
review under ADR 0018.

## Review Output And Closeout Boundary

A plan-review finding names the criterion, affected issue IDs, and the full
dependency path when the finding is about readiness. An approval says the exact
graph revision reviewed. The review does not prove implementation outcomes or
approve code for merge.

After execution, ordinary issues record local proof; validation issues record
independent classification and evidence. Epic and mission closeout map parent
Outcome claims to the accountable implementation, review, validation, and
evidence records. Open external prerequisites, unresolved decisions, missing
migration/docs/cleanup work, or incomplete validation keep closeout open even
if the main implementation change is complete.
