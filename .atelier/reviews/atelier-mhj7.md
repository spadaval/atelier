---
schema: atelier.review
schema_version: 2
id: atelier-mhj7
title: finding on atelier-n7c4 @30c2cab5
target: atelier-n7c4
identity: grok-reviewer (grok-4.6)
commit: 30c2cab59ec50f31ff32c9a697408721e039c77f
created_at: 2026-11-09T05:51:24.476273724Z
event:
  event: finding
  recommendation: defer
---

low: skills/agent-factory/procedures/orchestrate.md:119 The markdown link to plan.md#later-increments-and-strategy-changes puts all of plan.md (158 lines) on the orchestrator read path, including brief-the-human and two planning reviews, while the same paragraph already restates one independent planning review and no brief. A cold orchestrator reading plan.md from the top can still brief the human for a later increment, which is human work inside the mission. Drop the link and keep the restated sentence, plus one line that a strategy change goes back to a planner.
