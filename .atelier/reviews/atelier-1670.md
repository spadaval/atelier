---
schema: atelier.review
schema_version: 2
id: atelier-1670
title: '.agents/skills/agent-factory/references/strategic-plans.md:39 (template) and :78 (example) still present `mission: <mission ID>` as standard strategy front matter, and the strategy-to-mission link guidance there was not updated. docs/product/mission-authoring.md now says the mission''s path is canonical and the strategy need not name its mission. Agents following the skill template (the template is what put `mission:` in astromech''s strategy) will keep writing the key, and every re-import with a new mission ID will still need a hand edit of the strategy, which is the circular edit this record is meant to remove. Fix: mark `mission:` optional in the template (or drop it), and say there that the mission''s strategy path is the canonical link, set with `issue create/update --strategy`.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: 5ea0b021f0cd536f14bcbb311766fd499f2f023d
created_at: 2026-10-15T23:56:14.754469204Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    recorded_at: 2026-10-15T23:59:22.568950322Z
    follow_up: atelier-92mz
    reason: ADR 0020 forbids editing the vendored skill locally; the strategic-plans template change goes upstream with the other adoption changes in atelier-92mz
    migrated_from: atelier-syvs
---

.agents/skills/agent-factory/references/strategic-plans.md:39 (template) and :78 (example) still present `mission: <mission ID>` as standard strategy front matter, and the strategy-to-mission link guidance there was not updated. docs/product/mission-authoring.md now says the mission's path is canonical and the strategy need not name its mission. Agents following the skill template (the template is what put `mission:` in astromech's strategy) will keep writing the key, and every re-import with a new mission ID will still need a hand edit of the strategy, which is the circular edit this record is meant to remove. Fix: mark `mission:` optional in the template (or drop it), and say there that the mission's strategy path is the canonical link, set with `issue create/update --strategy`.
