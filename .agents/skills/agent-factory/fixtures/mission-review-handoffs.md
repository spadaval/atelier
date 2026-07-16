# Mission Authorship, Review, and Orchestration Handoff Fixtures

These are inspection fixtures for the Agent Factory guidance. They demonstrate
the role boundary required for a mission graph; they are not a substitute for
an Atelier lifecycle event or first-class evidence in a live repository.

## Supported Lifecycle: Draft to Execution

```text
Planner handoff
Repository: /repo/atelier
Mission: atelier-fixture-handoff
Authored graph revision: mission-plan-graph-v1:sha256:2d9c6a...
Author/material editors: fixture-planner, fixture-graph-editor
State reported by Atelier: draft; review requested
Planner decision: leave the authored graph in draft. No approval, ready
  transition, or execution assignment is claimed.
Next owner: separately assigned mission reviewer.

Mission-review assignment
Repository: /repo/atelier
Mission ID: atelier-fixture-handoff
Exact graph revision: mission-plan-graph-v1:sha256:2d9c6a...
Complete graph scope: mission, roots atelier-fixture-contract and
  atelier-fixture-lifecycle, all reachable descendants, and external
  prerequisite fixture-provider-contract.
Evidence destination: issue/atelier-fixture-handoff
Independence context: authors/editors fixture-planner, fixture-graph-editor;
  reviewer fixture-independent-reviewer.
Independence requirement: fixture-independent-reviewer is distinct from every
  author and material editor.

Reviewer result recorded by Atelier: current independent approval for
  mission-plan-graph-v1:sha256:2d9c6a...

Orchestrator readiness check
Atelier reports: mission atelier-fixture-handoff has current independent
  approval for mission-plan-graph-v1:sha256:2d9c6a... and is ready.
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
