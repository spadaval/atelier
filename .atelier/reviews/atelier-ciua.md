---
schema: atelier.review
schema_version: 2
id: atelier-ciua
title: '.atelier/issues/atelier-bw0b.md Scope still reads ''The ADR and AGENTS.md only. Product docs and code change in the follow-up issues'', but the branch also changes CLAUDE.md, .claude/settings.json, docs/index.md and docs/product/development-setup.md, a product doc. Failure: the owner record, the durable source for what this branch is allowed to touch, contradicts the diff that merges under it. The widening is recorded only in disposition events. Fix: update Scope (and the Done when if CLAUDE.md and hooks are now part of it).'
target: atelier-bw0b
identity: Independent reviewer 2 (Claude subagent)
commit: 0db4b1845dab4da518fe334854b54f50a9a95345
created_at: 2026-10-16T23:50:25.932017149Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (author)
    commit: d1c3b1bfb6394c6b82fc6978816052304591380a
    recorded_at: 2026-10-16T23:52:17.958696485Z
    resolution_commit: d1c3b1bfb6394c6b82fc6978816052304591380a
    reason: bw0b Scope lists every file the branch changes
    migrated_from: atelier-2q4j
---

.atelier/issues/atelier-bw0b.md Scope still reads 'The ADR and AGENTS.md only. Product docs and code change in the follow-up issues', but the branch also changes CLAUDE.md, .claude/settings.json, docs/index.md and docs/product/development-setup.md, a product doc. Failure: the owner record, the durable source for what this branch is allowed to touch, contradicts the diff that merges under it. The widening is recorded only in disposition events. Fix: update Scope (and the Done when if CLAUDE.md and hooks are now part of it).
