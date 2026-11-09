---
schema: atelier.review
schema_version: 2
id: atelier-rcfl
title: finding on atelier-ss4y @ca39f2ff
target: atelier-ss4y
identity: grok-reviewer (grok-4.6)
commit: ca39f2ff4f4903bc4c1c961b6d29077e0090ad48
created_at: 2026-11-09T06:19:34.374845246Z
event:
  event: finding
  recommendation: fix_now
---

R2 medium: skills/agent-factory/SKILL.md:83 Independent Review's No reviewer bullet still says to ask the user to review or arrange it, with no mission exception. An implementer inside an approved mission whose reviewer cannot start is sent to the human instead of escalating to the orchestrator; an orchestrator gets a mid-mission wait instead of settling availability or aborting. This fails atelier-pxme Done when 2 and contradicts orchestrate.md:14-19. The pxme Report calls it lone-agent guidance, but that condition is not in the rule. Inside a mission, escalate or abort; keep the user-review path outside one.
