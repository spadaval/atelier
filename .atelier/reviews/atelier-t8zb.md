---
schema: atelier.review
schema_version: 2
id: atelier-t8zb
title: Installed ~/.cargo/bin/atelier rejects status --hook claude, so .claude/settings.json's SessionStart hook fails until the CLI is reinstalled after merge.
target: atelier-sdb0
identity: Independent reviewer (Claude subagent)
commit: cbb151fe4909ae92ee60a07ae384abe9dda975f2
created_at: 2026-10-17T02:41:20.830438253Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: cbb151fe4909ae92ee60a07ae384abe9dda975f2
    recorded_at: 2026-10-17T02:41:22.225933216Z
    reason: No code change; the orchestrator reinstalls the CLI with cargo install --path crates/atelier-cli immediately after merge, as development-setup.md requires.
    migrated_from: atelier-dmhm
---

Installed ~/.cargo/bin/atelier rejects status --hook claude, so .claude/settings.json's SessionStart hook fails until the CLI is reinstalled after merge.
