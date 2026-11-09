---
schema: atelier.review
schema_version: 2
id: atelier-vjuy
title: finding on atelier-n7c4 @30c2cab5
target: atelier-n7c4
identity: codex-reviewer (gpt-6.1-sol)
commit: 30c2cab59ec50f31ff32c9a697408721e039c77f
created_at: 2026-11-09T05:48:08.718197090Z
event:
  event: finding
  recommendation: fix_now
---

medium: skills/agent-factory/procedures/orchestrate.md:116 The unconditional clause "and so does a current hazard" adds an abort condition broader than Before Starting and the strategy: it requires aborting the entire mission even when stopping the harmful work and correcting its route is safely within Authority. For example, an unexpectedly resource-heavy local worker can be stopped and rerun on the approved compute host without changing the outcome or strategy; this sentence instead sends the mission back to the human and planner. That defeats the stated requirement to settle what the orchestrator can and recreates avoidable human work. Remove the blanket hazard clause and let the complete-or-abort rule govern, or limit it to a hazard that cannot be safely settled within the strategy. Stop harmful work immediately in either case.
