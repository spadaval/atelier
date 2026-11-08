---
schema: atelier.review
schema_version: 2
id: atelier-2rlr
title: finding on atelier-l58v @f684caa1
target: atelier-l58v
identity: grok-plan-reviewer (grok-4.7)
commit: f684caa15b2bb4fe2a68878e52a27c3499a1d9df
created_at: 2026-11-08T05:49:45.285232253Z
event:
  event: finding
  recommendation: fix_now
---

medium: The atelier-qj9d limit line drops a limit this edit will hit. That item's Done when is at most 1,000 lines for the agent-facing skill, SKILL.md at most 100, each other role's read path at most 300 and the orchestrator's 450, counted at the ~80-column wrap. A role's path is SKILL.md, its procedure, and every file the procedure links, counted whole, plus the Atelier reference when the role runs atelier (.atelier/issues/atelier-qj9d.md Done when). This line's parenthesis omits the SKILL.md cap and the counting rule. SKILL.md is 96 lines, and this item adds a gardener entry to it. Include the 100-line cap and the counting rule, or the overrun note will not mention them.
