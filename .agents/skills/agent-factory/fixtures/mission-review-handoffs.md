# Mission Authorship, Review, and Orchestration Handoff Fixtures

These are static inspection fixtures for the Agent Factory guidance. They
demonstrate the role boundary required for a mission graph; they are not a
substitute for an Atelier lifecycle event or first-class evidence in a live
repository. Live lifecycle behavior remains deferred to `atelier-72k4`.

## Supported Lifecycle: Draft to Execution

```text
Planner handoff
Repository: /repo/atelier
Mission: atelier-fixture-complete
Authored graph revision: mission-plan-graph-v1:sha256:5f2a625447f52a44e6b17ca868c54128a6d98db78ded4a0d245268a355cab40b
Author/material editors: fixture-planner, fixture-graph-editor
State reported by Atelier before a review request: draft
Planner decision: leave the authored graph in draft. No approval, ready
  transition, or execution assignment is claimed.
Next owner: separately assigned mission reviewer.

Repository review-request transition
Atelier transition: draft -> plan_review
State reported by Atelier after the request: plan_review; review requested

Mission-review assignment
Repository: /repo/atelier
Mission ID: atelier-fixture-complete
Exact graph revision: mission-plan-graph-v1:sha256:5f2a625447f52a44e6b17ca868c54128a6d98db78ded4a0d245268a355cab40b
Complete graph scope: mission; roots atelier-fixture-contract,
  atelier-fixture-lifecycle, and atelier-fixture-validation; reachable IDs
  atelier-fixture-contract, atelier-fixture-digest, atelier-fixture-events,
  atelier-fixture-contract-integration, atelier-fixture-lifecycle,
  atelier-fixture-ready-gate, atelier-fixture-start-gate,
  atelier-fixture-migration, atelier-fixture-docs, atelier-fixture-cleanup,
  atelier-fixture-validation, and atelier-fixture-closeout; external
  prerequisites none.
Evidence destination: issue/atelier-fixture-complete
Independence context: authors/editors fixture-planner, fixture-graph-editor;
  reviewer fixture-independent-reviewer.
Independence requirement: fixture-independent-reviewer is distinct from every
  author and material editor.

Reviewer result recorded by Atelier: current independent approval for
  mission-plan-graph-v1:sha256:5f2a625447f52a44e6b17ca868c54128a6d98db78ded4a0d245268a355cab40b

Orchestrator readiness check
Atelier reports: mission atelier-fixture-complete has current independent
  approval for mission-plan-graph-v1:sha256:5f2a625447f52a44e6b17ca868c54128a6d98db78ded4a0d245268a355cab40b and is ready.
Orchestrator decision: execution assignments may begin.
```

The planner cannot replace the reviewer result. If a material graph edit occurs
after approval, the approval is stale and the reviewer must inspect the new
exact revision before Atelier can report the mission ready again.

## Unsupported Lifecycle: Readiness Gap

```text
Repository: /repo/no-plan-review-lifecycle
Install/readiness inspection: tracker can show a mission and issue status, but
  cannot retain an authored draft, bind a review to an exact revision, attribute
  reviewer independence, or report current approval and ready state.
Result: not fully operable for Agent Factory mission planning.
Next action: record the missing lifecycle capability in the repository tracker;
  do not claim approval or simulate readiness in Agent Factory guidance.
```
