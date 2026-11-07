---
schema: atelier.review
schema_version: 2
id: atelier-3w6e
title: finding on atelier-n0qh @1dc540c6
target: atelier-n0qh
identity: lighter-workflow reviewer for atelier-n0qh (claude-opus-5-5)
commit: 1dc540c65e66c1445b741d4920a979592d02b7e2
created_at: 2026-11-07T02:19:16.124983764Z
event:
  event: finding
  recommendation: fix_now
---

low: Agent Factory skills/agent-factory/procedures/orchestrate.md:177 (agents-skills branch feature/atelier-n0qh, d0e645f) says 'Validate each merged item's open after-merge checks, or carry them to an item outside the mission, before closing it.' That offers carrying as an unconditional equal of validating, which an orchestrator can read as 'validation can wait: carry it and the warning goes away'. The human's rule allows carrying, but the text should limit it: validate every check that can be validated now; carry only a failed check (to its follow-up bug) or one that needs time and real use beyond the mission, and name each carried check in the mission's close report for the human. Same for the atelier.md and tracker-records.md lines that present 'carried to' without saying when.
