---
schema: atelier.review
schema_version: 2
id: atelier-huh0
title: finding on atelier-ss4y @ca39f2ff
target: atelier-ss4y
identity: codex-reviewer (gpt-6.1-sol)
commit: ca39f2ff4f4903bc4c1c961b6d29077e0090ad48
created_at: 2026-11-09T06:16:01.754959181Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium: skills/agent-factory/SKILL.md:83-87 The No reviewer rule still universally says to ask the user to review or arrange review, and offers a knowingly unreviewed merge, with no mission exception. An implementer in an approved mission whose reviewer is unavailable is sent directly to the human instead of escalating to its orchestrator; an orchestrator gets a mid-mission wait instead of settling the problem or aborting. This fails atelier-pxme Done when 2 and contradicts orchestrate.md:14-19. The pxme Report calls this lone-agent/outside-mission guidance, but the condition is absent from the actual rule. Qualify it: inside a mission escalate to the orchestrator, which resolves review availability or aborts under Before Starting; preserve the existing user-review path outside a mission.
