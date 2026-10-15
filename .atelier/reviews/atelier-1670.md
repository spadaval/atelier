---
schema: atelier.review
schema_version: 1
id: atelier-1670
title: '.agents/skills/agent-factory/references/strategic-plans.md:39 (template) and :78 (example) still present `mission: <mission ID>` as standard strategy front matter, and the strategy-to-mission link guidance there was not updated. docs/product/mission-authoring.md now says the mission''s path is canonical and the strategy need not name its mission. Agents following the skill template (the template is what put `mission:` in astromech''s strategy) will keep writing the key, and every re-import with a new mission ID will still need a hand edit of the strategy, which is the circular edit this record is meant to remove. Fix: mark `mission:` optional in the template (or drop it), and say there that the mission''s strategy path is the canonical link, set with `issue create/update --strategy`.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: 5ea0b021f0cd536f14bcbb311766fd499f2f023d
created_at: 2026-10-15T23:56:14.754469204Z
event:
  event: finding
  recommendation: fix_now
---

.agents/skills/agent-factory/references/strategic-plans.md:39 (template) and :78 (example) still present `mission: <mission ID>` as standard strategy front matter, and the strategy-to-mission link guidance there was not updated. docs/product/mission-authoring.md now says the mission's path is canonical and the strategy need not name its mission. Agents following the skill template (the template is what put `mission:` in astromech's strategy) will keep writing the key, and every re-import with a new mission ID will still need a hand edit of the strategy, which is the circular edit this record is meant to remove. Fix: mark `mission:` optional in the template (or drop it), and say there that the mission's strategy path is the canonical link, set with `issue create/update --strategy`.
