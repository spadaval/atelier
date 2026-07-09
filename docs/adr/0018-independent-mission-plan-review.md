# ADR 0018: Independent Mission-Plan Review Gates Execution

Status: Accepted
Date: 2026-07-09

## Context

Atelier currently allows a configured mission to move directly from `draft` to
`ready` when its sections parse. That rule came from atelier-a44d,
atelier-ql9k, and atelier-1mga: missions use repository workflow policy rather
than a hidden lifecycle, planning starts from a worker-usable `Outcome`, and
planners do not prewrite validation transcripts. Those choices reduced
ceremony, but they leave one high-impact judgment with the author: whether the
mission's complete issue graph is safe and sufficient to execute.

Mission-plan review needs findings, approval, authorship, and freshness, but it
is not review of a code branch. Reusing the issue `review` field or treating a
plan approval as evidence would conflate planning readiness with code merge or
outcome proof. Approval also cannot be a mutable Boolean: it is meaningful only
for the exact graph inspected by a reviewer independent of its authors.

## Decision

### Lifecycle and authority

The repository's configured mission planning path is
`draft -> plan_review -> ready -> in_progress`.

- `draft` is authored planning and is not executable. Its advancing work is not
  offered as executable mission work merely because local issue statuses look
  ready.
- requesting review moves the mission to `plan_review`, whose status role is
  `reviewer`. Findings, change requests, resolutions, and approval are recorded
  against an exact graph revision. Repairs may occur while review remains open.
- moving to `ready` requires a current independent approval, no unresolved
  blocking finding or later change request, and every configured readiness
  validator. `start` rechecks approval freshness and declared dependency
  closure so a ready-state edit cannot bypass the gate.
- `in_progress` begins execution. Internal advancing work need not already be
  terminal; it is the work the mission is starting to coordinate.

These statuses, transitions, validators, and actions remain declared in
`.atelier/workflow.yaml` under ADR 0015. Atelier adds bounded capabilities for
mission-plan provenance, revision calculation, and readiness validation; it
does not add a hidden mission lifecycle or a general user-defined approval
engine. The live workflow and canonical records change atomically with the
implementation and migration. Contract-first documentation does not install an
unsupported validator into the pre-cutover workflow.

### Canonical revision

A mission graph revision is a versioned deterministic digest. It includes:

| Input | Included content |
| --- | --- |
| Mission | Stable ID, issue type, title, priority, labels, all authored planning sections, and direct `advances` roots. |
| Reachable work | Every unique issue reachable from an `advances` root through canonical hierarchy, including stable ID, issue type, title, priority, labels, and all authored planning sections. |
| Execution graph | Canonical root membership, hierarchy edges, `advances` edges, and workflow-driving blocker/dependency edges among or from reviewed records, including external prerequisite IDs. |
| Review contract | Revision-algorithm version so a change to canonical inputs cannot silently preserve old approval. |

Canonical ordering is by stable identity and relationship tuple, not file or
authoring order. Workflow status, timestamps, branch/review fields, activity,
notes, evidence attachments or receipts, code commits, cache state, and
context-only `relates` links are excluded. They describe execution, proof, or
context rather than the plan being judged.

A material edit changes an included input. This covers mission intent,
`Outcome`, non-scope, risks, scope roots, reachable issue content, issue type,
priority or labels, adding or removing reachable work, hierarchy, declared
blockers/dependencies, and validation or closeout coverage. A non-material edit
changes only an excluded input, such as a note, activity entry, workflow status,
evidence receipt, review event, timestamp, cache repair, or code commit.

An approval stores the digest it reviewed. A material edit produces a different
digest and makes the prior approval stale without deleting or rewriting its
history. Non-material activity preserves freshness.

### Independence, findings, and provenance

The mission plan author is the actor accountable for the initial reviewed
revision. A material editor is every actor whose attributed material changes
contribute to the current revision. The approving reviewer must have a stable
actor identity distinct from the author and every material editor for that
revision. Role names, provider bot accounts, or a changed display name do not
make the same actor independent.

Missing or untrusted authorship is not independence. A direct record edit that
changes the digest without corresponding durable attribution makes provenance
incomplete and blocks approval until the edit is explicitly attributed; Atelier
must not infer or fabricate an identity.

Mission-plan review uses typed canonical events in the mission's existing
`.atelier/issues/<mission-id>.activity/` sidecar stream; it does not add a
second mutable review snapshot. The state is rebuilt from those tracked,
append-only events. Events include request, material-edit attribution, blocking
or non-blocking finding, change request, resolution, approval, and
migration/grandfathering. Each decision event names its actor and graph
revision; findings also name affected issue IDs and, when applicable, the
dependency path. A resolution records disposition but does not approve the
graph. Approval is allowed only when the actor is independent, the revision is
current, blocking findings are resolved, and no later change request remains
effective.

The event vocabulary may share implementation with native review-room events,
but mission-plan state is not stored in the issue `review` field and does not
open, approve, complete, or merge a branch review artifact.

### Migration

Migration is deterministic and idempotent and never invents reviewer approval,
authorship, or dependency completeness.

| Legacy mission state | Cutover treatment |
| --- | --- |
| `draft` | Remains `draft`, unapproved and non-executable; request independent review normally. |
| `ready` | Moves to `plan_review` with no approval. It must receive current independent approval before returning to `ready`. |
| active / `in_progress` | Remains active with an explicit legacy-grandfather event bound to its exact cutover revision. Work already in that revision may continue. A material graph edit makes the grandfather stale and blocks not-yet-started graph work until the current revision is independently approved. |
| configured terminal state | Remains unchanged as historical state. No retroactive approval is fabricated or required. |

Unknown or malformed legacy states fail migration with recovery guidance rather
than being treated as ready. Re-running migration produces the same canonical
state, and rebuilding the domain cache preserves provenance, freshness, and the
grandfather boundary.

### Boundary from other judgments

| Judgment | Owner and artifact | Effect |
| --- | --- | --- |
| Mission-plan readiness | Independent mission reviewer; revision-bound plan-review events | Permits the configured transition to `ready`; never merges code or proves outcomes. |
| Code review | Epic or other branch owner's native/provider review artifact | Judges a changeset and may authorize merge; never makes a mission plan ready. |
| Outcome validation | Independent validator; validation issue and evidence receipts derived from `Outcome` | Classifies implemented claims; it is not plan approval. |
| Merge authority | `atelier review merge` or configured provider | Integrates reviewed code; it does not transition mission planning state. |

Mission-plan review judges the whole issue set: Outcome clarity, coverage,
decomposition, dependencies, sequencing, external prerequisites, validation and
closeout ownership, and safe initial parallelism. This amends the
atelier-ql9k Outcome-only readiness question, but retains Outcome-led planning:
planners still do not prewrite validator transcripts or evidence receipts.

## Reconciliation With Prior Decisions

| Prior decision | Classification | Current rule |
| --- | --- | --- |
| atelier-a44d: lifecycle is simple and workflow-owned; no mission review state | Retained in ownership, superseded in state set | Mission policy remains configured rather than hidden, but this repository now configures `plan_review`; no arbitrary workflow engine or second parent acceptance-review issue type is introduced. |
| atelier-ql9k: readiness asks whether `Outcome` is understandable; planners do not author validation paperwork | Amended and retained | Outcome clarity remains necessary and planner-authored proof paperwork remains rejected. Independent review additionally judges completeness and executability of the full issue graph. |
| atelier-1mga: workflow ownership, `advances` scope, Outcome-led plans, validator-derived proof, and evidence-as-receipt | Retained except direct readiness | Those ownership and proof boundaries remain. The direct `draft -> ready` path and author-only readiness judgment are superseded by exact-revision independent review. |

## Durable Source Matrix

| Source | Decision owned here |
| --- | --- |
| `PRODUCT_INTENT.md` | Product reason for independent review and the target configured lifecycle without planner-authored execution proof. |
| `CONTEXT.md` | Canonical terms for graph revision, material edit, independent reviewer, and mission-plan approval. |
| `docs/product/work-model.md` | Operator-visible issue-set judgment, freshness, dependency, migration, and plan/code/validation boundaries. |
| `docs/product/workflow-configuration.md` | Target `draft -> plan_review -> ready -> in_progress` statuses, roles, transitions, and validator placement; atomic cutover from the pre-implementation workflow. |
| `docs/architecture/markdown-first-record-store.md` | Canonical activity-sidecar event ownership and rebuild boundary without a mutable review snapshot. |
| `docs/architecture/quality/validation.md` | Mission-plan review is readiness, while validators derive and record outcome proof independently after implementation. |
| This ADR | Exact digest inputs, materiality, actor independence, finding and freshness rules, legacy-state migration, and supersession of incompatible prior decisions. |
| `.atelier/workflow.yaml` | Live executable policy. Before implementation it is explicitly marked pre-cutover; implementation must replace the legacy direct transition atomically with supported validators, canonical events, and migration. |

## Consequences

- Draft missions and their descendants cannot leak into executable work
  selection before approval.
- Review history and stale approvals remain inspectable rather than being
  overwritten.
- Existing ready missions pause for review; active missions receive a narrow,
  revision-bound compatibility rule instead of fabricated approval.
- Implementations must make direct edits, rebuilds, alternate transitions, and
  normal CLI projections obey the same provenance and freshness rules.
- atelier-2uim audits this reconciliation after the authoring standard lands;
  atelier-t876 independently validates executable and migration behavior after
  implementation.
