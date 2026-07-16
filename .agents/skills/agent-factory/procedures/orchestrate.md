# Orchestrate

Use this subskill when you are the primary coordinator for a mission, epic, or
multi-item workstream. The orchestrator scopes, delegates, integrates, reviews,
validates, checkpoints, and steers recovery.

## Operating Model

One agent, one role. Do not personally apply multiple role subskills to complete
delegated work. Spawn subagents for complex role-specific work and assign
exactly one Agent Factory subskill to each.

Use the repository's executable guidance for tactical details. In Atelier
repositories, start from `atelier man manager`, `atelier status`, focused
`atelier issue show <objective-id>`, and focused record drill-down commands.

## Orchestrator Responsibilities

- Establish the active mission, epic, or workstream and its durable scope.
- Check visible work readiness and worktree state before assigning mutating
  work.
- Resolve or block high-leverage choices before dependent implementation.
- Ensure each assigned item has clear scope, proof, evidence destination, and
  independence requirements.
- Delegate bounded worker, reviewer, validator, audit, docs, migration, and
  scout slices early enough that proof is durable.
- Integrate completed work into coherent checkpoints and preserve unrelated
  user or agent changes.
- Route high-risk diffs to `review` and scenario-centered claims to `validate`.
- Close parent work only after accountable child proof, required review or
  validation, and clean handoff state are visible through repository-owned
  product surfaces.

## Mission Authoring, Review, and Execution

Treat mission authorship, mission review, and orchestration as separate
handoffs. A planner creates or repairs an authored draft and requests review;
it does not approve its own graph, make that graph ready, or authorize
execution. Assign the exact draft to a separately assigned `mission-review`
agent, rather than substituting code `review` or outcome `validate`.

A mission-review assignment must extend the normal assignment block with all
of these inputs:

```text
Repository: <absolute path>
Mission ID: <mission-id>
Exact graph revision: <revision algorithm version and digest>
Complete graph scope: <mission, direct advances roots, every reachable
  hierarchy descendant, and workflow-driving internal/external prerequisites>
Evidence destination: <where revision-bound findings or approval are recorded>
Independence context: <initial author, every material editor, assigned reviewer>
Independence requirement: reviewer is independent of the author and every
  material editor
```

Do not fill in absent scope, revision, or provenance from private context. A
review finding returns the graph to a separately assigned planner/material
editor; a material edit invalidates the previous result. Before assigning
execution, use Atelier's current manager/status/mission surfaces to confirm the
exact mission has current independent approval and is reported ready. An
authored draft, an old approval, or a reviewer statement alone is not execution
authority. When Atelier does not report current approval and ready state, stop
and route the next action to the planner or reviewer outcome it reports.

## Assignment Block

Every delegated worker prompt must include:

```text
Repository: <absolute path>
Active mission: <mission-id or none>
Parent epic: <epic-id or none>
Workspace/branch context: <mission workspace, owner branch, or explicit exception>
Assigned issue(s): <exact tracker IDs>
Role/subskill: <exactly one Agent Factory subskill>
Model: <model choice>
Model rationale: <complexity, ambiguity, risk, review depth, and proof need>
Owned files/workflows: <paths, modules, commands, or workflows>
Out of scope: <files, commands, policies, or adjacent issues>
Expected proof: <observable command output, file content, test, transcript, or artifact>
Evidence destination: <issue note or first-class evidence target>
Independence requirement: <none, independent review, independent validation, epic, or mission>
Dirty worktree rule: preserve unrelated changes
Final handoff schema:
  result:
  issue ID:
  subskill:
  changed files:
  evidence IDs:
  commands run:
  dirty state:
  branch/commit:
  blockers:
  exact follow-up recommendation:
```

Name required docs, ADRs, glossary terms, known breakage, and parent validation
criteria when they affect the assignment.

## Model Routing

Before delegating, load
[Submodel Selection](../references/submodel-selection.md). Record the selected
model, reasoning effort, rationale, and any runtime fallback in every assignment
block.

## Handoff

Final orchestration handoff names completed work, commits or branches, closed
items, evidence records, validation commands, residual breakage, follow-up
items, visible readiness checks, and worktree state.
