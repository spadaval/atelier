# Factory Missions

## Source

- Article: `https://factory.ai/news/missions`

These notes are based on the public article as read on 2026-06-08.

## Relevant Ideas

- The product model is goal, scope approval, decomposition, execution, and
  validation. Atelier should make missions and milestones durable enough that a
  human can approve scope before work expands.
- Milestones are checkpoints, not just labels. Each one should end in a
  validation phase that reviews accumulated work, runs tests, checks
  regressions, and creates follow-up work before moving on.
- Fresh worker context per feature is a useful orchestration model. Atelier can
  support this with durable plans, links, worktrees, and evidence without
  storing every agent process as a first-class record yet.
- Selective parallelism is the important lesson. Parallelize where coordination
  cost is low; keep sequential gates where integration risk is high.
- Validation should include more than unit tests. UI flows, command scenarios,
  integration checks, and review evidence should all be representable.
- Mission Control should be an operator view of intent, progress, blockers,
  validation, and follow-up work. It should not require a rich UI before the
  underlying projection is useful.

## Do Not Copy Blindly

- Do not build a SaaS mission runner. Atelier remains local-first and Git-backed.
- Do not add direct agent lifecycle management in the current slice. The useful
  near-term target is durable mission structure, workflow configuration,
  worktree state, gates, and evidence.
- Do not imply that autonomous execution is complete when an agent command exits.
  Completion should be a workflow-defined handoff or validation result.

## Follow-Up Beads

- `atelier-v72a`: keep missions, milestones, plans, evidence, and gates
  first-class enough to support milestone validation.
- `atelier-kitl`: workflow config should express validation phases, gate
  requirements, and lightweight versus strict flows.
- `atelier-9h7g`: Mission Control projection should emphasize scope, progress,
  blockers, branches/worktrees, evidence, gates, validation, and follow-up work.

