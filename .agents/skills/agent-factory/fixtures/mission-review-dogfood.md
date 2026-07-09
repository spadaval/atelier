# Mission-Review Dogfood Packets

These packets are input fixtures for a separately assigned `mission-review`
agent. They are not approvals, findings, or implementation evidence. The
reviewer must inspect the named exact graph revision, record its own outcome in
the stated evidence destination, and remain read-only.

The canonical-manifest text below is a compact fixture representation, not an
alternative mission authoring format. Its `mission-plan-graph-v1` digest is the
review binding for this dogfood packet. The reviewer should treat the listed
records and edges as the complete graph scope; no unlisted context link supplies
missing scope or a prerequisite.

## Packet A: Exact-Revision Complete Case

```text
Repository: /root/.codex/worktrees/9661/atelier
Mission ID: atelier-fixture-complete
Exact graph revision: mission-plan-graph-v1:sha256:002869c1092e45a5e0bd40a5e2e56895094552f122231035d6449f98a829ccbc
Author: fixture-author
Material editors: fixture-author, fixture-editor
Assigned reviewer: fixture-independent-reviewer
Evidence destination: issue/atelier-l5mw (first-class evidence or durable note)

Mission Outcome: Operators can submit an independently reviewed mission graph
and execution cannot begin without current approval.
Mission non-scope: code review semantics.
Mission risks: actor identity and external prerequisites.
Direct advances roots: atelier-fixture-contract, atelier-fixture-lifecycle,
atelier-fixture-validation.

Reachable records:
- atelier-fixture-contract (epic): Define revision and event contract.
  Outcome: Canonical graph revisions and attributed review events have one
  public contract.
- atelier-fixture-digest (task): Calculate deterministic graph revision.
  Outcome: The graph digest covers all material planning inputs.
- atelier-fixture-events (task): Record attributed plan-review events.
  Outcome: Review events bind actor and revision.
- atelier-fixture-contract-integration (task): Integrate mission review
  contract. Outcome: The public contract coherently composes digest and events.
- atelier-fixture-lifecycle (epic): Enforce review-gated mission lifecycle.
  Outcome: Only current independent approval permits ready.
- atelier-fixture-ready-gate (task): Gate ready transition.
  Outcome: Ready rejects stale or non-independent approval.
- atelier-fixture-validation (validation): Independently classify mission
  review behavior. Outcome: Validation maps public lifecycle claims and
  evidence.

Hierarchy: contract -> digest, events, contract-integration; lifecycle ->
ready-gate.
Dependencies: digest -> contract-integration; events -> contract-integration;
contract-integration -> ready-gate; ready-gate -> validation.
External prerequisites: none.
```

The independent reviewer must apply the full rubric and issue either
revision-bound approval or findings. The author and material editor are
explicitly prohibited from supplying that approval.

## Packet B: Defective and Self-Approval Rejection Case

```text
Repository: /root/.codex/worktrees/9661/atelier
Mission ID: atelier-fixture-defective
Exact graph revision: mission-plan-graph-v1:sha256:613018d218cebe89c42cf184e4af4706fae212cf4667a163526efd25049a9fce
Author: fixture-author
Material editors: fixture-author
Assigned reviewer: fixture-author
Evidence destination: issue/atelier-l5mw (first-class evidence or durable note)

Mission Outcome: The revised workflow is safe.
Mission non-scope: code review semantics.
Mission risks: provider contract.
Direct advances roots: atelier-fixture-ready-gate, atelier-fixture-schema,
atelier-fixture-cache, atelier-fixture-validate.

Reachable records:
- atelier-fixture-ready-gate (task): Gate ready transition.
  Outcome: Ready rejects invalid approval. Notes: Depends on provider contract.
- atelier-fixture-schema (task): Revise canonical schema.
  Outcome: Schema supports revision events.
- atelier-fixture-cache (task): Revise cache rebuild.
  Outcome: Cache rebuild supports revised schema.
- atelier-fixture-validate (validation): Validate review behavior.
  Outcome: Tests pass.

Hierarchy: none.
Dependencies: none declared.
External prerequisites: none declared.
```

The assignee must not approve this packet because its actor is the author and a
material editor. A distinct reviewer must return a read-only, revision-bound
report with finding IDs, affected issue IDs, and dependency paths where the
rubric requires them; repairs go to a separately assigned planner/material
editor.
