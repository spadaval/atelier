---
schema: atelier.review
schema_version: 1
id: atelier-46ez
title: 'medium: .agents/skills/agent-factory/SKILL.md:92-93, procedures/orchestrate.md:29-32,66,80,97, references/finding-disposition.md:14 (and constitution.md:36 ''The agent that wrote code''). Statements of the review rule for the orchestrator''s own work and the integration gate still say code: ''Code the orchestrator writes itself needs review like anyone else''s'', ''review and validation of code you wrote must be [independent]'', ''Code you write yourself follows the same review rule'', ''For findings on code you wrote yourself'', ''an orchestrator''s own code'', and ''Unreviewed code may be integrated into a working or mission branch ... but it does not reach the default or protected branch until review is done'' (SKILL.md:73 changed the same sentence to ''Unreviewed work''). Impact: Done when requires every statement limiting review to code to say non-trivial changes; the receipt atelier-6icy greps only the phrasings that were changed, so it could not catch these. Failure scenario: an orchestrator writes a strategy doc or ADR itself (how S8''s unreviewed prose arose) and reads the orchestrate rules scoped to code, so it treats its own planning prose as outside the review and merge gate. Fix: say work/changes instead of code in these lines (keep ''code'' where the reading is code-specific, such as good-code and review.md''s code reading).'
target: atelier-aufe
identity: Independent reviewer (Claude subagent)
commit: 1b3160c4d463d4996f3ab8b806e3930dcda0aec9
created_at: 2026-10-23T00:10:00.713759438Z
event:
  event: finding
  recommendation: fix_now
---

medium: .agents/skills/agent-factory/SKILL.md:92-93, procedures/orchestrate.md:29-32,66,80,97, references/finding-disposition.md:14 (and constitution.md:36 'The agent that wrote code'). Statements of the review rule for the orchestrator's own work and the integration gate still say code: 'Code the orchestrator writes itself needs review like anyone else's', 'review and validation of code you wrote must be [independent]', 'Code you write yourself follows the same review rule', 'For findings on code you wrote yourself', 'an orchestrator's own code', and 'Unreviewed code may be integrated into a working or mission branch ... but it does not reach the default or protected branch until review is done' (SKILL.md:73 changed the same sentence to 'Unreviewed work'). Impact: Done when requires every statement limiting review to code to say non-trivial changes; the receipt atelier-6icy greps only the phrasings that were changed, so it could not catch these. Failure scenario: an orchestrator writes a strategy doc or ADR itself (how S8's unreviewed prose arose) and reads the orchestrate rules scoped to code, so it treats its own planning prose as outside the review and merge gate. Fix: say work/changes instead of code in these lines (keep 'code' where the reading is code-specific, such as good-code and review.md's code reading).
