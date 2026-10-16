# ADR 0018: Strategy, Draft Epics, And Adaptive Expansion

## Status

Accepted. Partially superseded by
[ADR 0021](0021-parent-issues-replace-epics.md): epics became parent issues,
and draft epics became `backlog` work.

## Context

Atelier originally assumed that a mission could be decomposed into detailed
work before execution. That works for bounded changes but fails for long-running
missions whose later route depends on evidence produced by earlier increments.
The replacement model must give Managers authority to adapt without making
strategy ephemeral or turning Atelier into a plan-approval system.

Several tempting representations create the wrong ownership boundary:

- putting a backlog of anchored outcomes or user stories in a strategy document;
- treating epics as engineering packages with no outcome contract;
- defining separate product and engineering epic types with the same lifecycle;
- requiring validation profiles before the intended increment is understood;
- representing replanning through approvals, graph versions, or dedicated
  records; and
- treating every todo-category record without blockers as executable work.

Atelier already has the useful shapes: durable repository Markdown, missions,
epics, issues, workflow state, typed relationships, evidence, activity notes,
and Git history. The design should use them directly.

## Decision

1. Strategy is governing mission intent.
   A mission-scoped strategy document owns the finished outcome, target system,
   governing decisions and tradeoffs, boundaries, adaptation authority, and
   mission-level assurance. It is ordinary versioned Markdown, not a product
   backlog, first-class plan record, or user-story container.

2. One epic type represents outcome-bearing delivery increments.
   An epic owns a desired result that can be implemented, reviewed, validated,
   and integrated as one coherent branch. Its outcome may be product-facing or
   internal: system, architecture, operational, quantitative, destructive, or
   behavior-preserving work uses the same epic shape.

3. Draft epics form the provisional outcome map.
   A draft epic must state a meaningful preliminary outcome and the uncertainty
   preventing responsible execution. It may omit children, an implementation
   route, exact dependencies, an Evidence section, and exact validation
   criteria. Draft means structurally valid but not executable.

4. Expansion is the transition from draft planning to ready execution.
   Expansion refines the epic outcome and preservation claims, creates only the
   immediate evidence-backed child work, represents known dependencies, and
   selects an appropriate proof route. The ordinary workflow transition to
   `ready` records that the epic is executable; there is no separate `expanded`
   field or plan lifecycle.

5. Ready-work projection follows executable workflow transitions.
   A todo-category status alone does not make work ready. Work is eligible for
   ready selection only when its current workflow status exposes a direct
   transition into active work and its dependency context permits execution.
   Draft statuses therefore remain visible planning state without entering the
   ready queue.

6. Proof planning is just in time.
   Strategy states mission-level assurance and durable risk boundaries. Exact
   epic and issue proof is chosen during expansion, when the increment is known.
   Repository defaults remain sufficient for ordinary work; custom Evidence
   prose or validation issues are added only when the claim or risk requires
   them.

7. Replanning mutates the graph directly.
   Managers may create, split, resequence, block, revise, or supersede work
   within active strategy. Git, tracker history, preserved evidence, and an
   optional mission note explain material changes. Atelier does not add a
   replanning record, approval, identity, digest, or graph freeze.

## Consequences

- The starter epic workflow gains `draft -> ready -> in_progress` planning and
  execution states.
- Ready queues and mission views must stop treating all todo-category work as
  executable.
- Draft epics may be created without placeholder Evidence text.
- Mission views must distinguish the draft outcome map from the current ready
  and active horizon.
- Product and engineering outcomes remain tied to delivery and validation
  without duplicating epic types.
- Strategy revision remains distinct from implementation replanning: changing
  mission outcome, target system, governing tradeoffs, boundaries, assurance,
  or adaptation authority requires a strategy revision; reshaping epics and
  issues within those bounds does not.

## Alternatives Considered

### Anchored Outcomes In Strategy Markdown

Rejected as the default because a strategy would become a brittle shadow
tracker. Independently managed user stories or requirements should be issue
records when they genuinely need their own lifecycle.

### Product Epics And Engineering Epics

Rejected because both shapes would own the same branch, review, child-work, and
validation lifecycle. Labels or authored outcomes can provide orientation
without duplicating the domain model.

### Outcome Epics With A New Increment Record

Deferred because it would add hierarchy, workflow, branch-ownership, review,
and migration machinery. Reconsider only if repeated missions cannot express
valuable, independently validated outcomes at one-branch epic scale.

### Validation Profiles

Deferred because validation follows the actual claim and risk. Reusable proof
bundles may be introduced after repeated mechanically identical scenarios
demonstrate value, not as a product-versus-engineering classifier.

### Dedicated Replanning Records

Rejected because the current graph, evidence, activity notes, and Git history
already preserve the facts needed for continuation. A mandatory record would
add ceremony without improving authority or proof.
