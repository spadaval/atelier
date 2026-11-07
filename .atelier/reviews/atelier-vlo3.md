---
schema: atelier.review
schema_version: 2
id: atelier-vlo3
title: '.agents/skills/agent-factory/references/atelier.md:28-31: still tells agents to revise epic/issue bodies by hand-editing .atelier/issues/<id>.md and says ''issue update --body applies only to missions''. That is now false and contradicts item 4. Agents follow this file, so they will keep hand-editing. Fix: point to ''atelier issue update <id> --body'' for every type (Notes and Close Reason kept) and drop the missions-only clause.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:43.895391131Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 46a88322a43ee881e37fab36f7a56613f32f0c87
    recorded_at: 2026-10-15T23:50:28.123717Z
    follow_up: atelier-92mz
    reason: ADR 0020 forbids editing the vendored skill locally; the upstream change and resync are tracked in atelier-92mz with the other adoption changes
    migrated_from: atelier-giwo
---

.agents/skills/agent-factory/references/atelier.md:28-31: still tells agents to revise epic/issue bodies by hand-editing .atelier/issues/<id>.md and says 'issue update --body applies only to missions'. That is now false and contradicts item 4. Agents follow this file, so they will keep hand-editing. Fix: point to 'atelier issue update <id> --body' for every type (Notes and Close Reason kept) and drop the missions-only clause.
