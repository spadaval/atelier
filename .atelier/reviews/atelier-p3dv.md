---
schema: atelier.review
schema_version: 2
id: atelier-p3dv
title: 'low: crates/atelier-cli/src/commands/git.rs:683-700 dropping the scope active count loses a fact for parent-owned branches. Live repo: ''feature/atelier-9g48  atelier-9g48 todo  Show which waiting branches...'' reads as an idle branch although its child atelier-ftqn is done on it; before, ''| N active'' showed scope activity. The issue only called ''0 active'' noise for a done owner. Impact: an orchestrator scanning Branches cannot tell a parent-owned branch with work in progress from an untouched one. Suggested: let atelier-tciw''s readiness facts (or a scope cue such as ''1 active'' when non-zero) cover parent-owned branches; defer to tciw.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: 57aaf3090f1be8100a33932f28e2c6aea6376cf3
created_at: 2026-10-22T20:43:04.609635080Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: f22da064b6220ed8ebc0a1b233564c6d1d5ee0e0
    recorded_at: 2026-10-22T20:43:35.526704103Z
    follow_up: atelier-tciw
    reason: Showing activity under a parent-owned branch is a per-branch readiness fact; atelier-tciw builds those facts for the Branches rows.
    migrated_from: atelier-2u37
---

low: crates/atelier-cli/src/commands/git.rs:683-700 dropping the scope active count loses a fact for parent-owned branches. Live repo: 'feature/atelier-9g48  atelier-9g48 todo  Show which waiting branches...' reads as an idle branch although its child atelier-ftqn is done on it; before, '| N active' showed scope activity. The issue only called '0 active' noise for a done owner. Impact: an orchestrator scanning Branches cannot tell a parent-owned branch with work in progress from an untouched one. Suggested: let atelier-tciw's readiness facts (or a scope cue such as '1 active' when non-zero) cover parent-owned branches; defer to tciw.
