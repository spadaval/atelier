---
schema: atelier.review
schema_version: 2
id: atelier-a7b0
title: setup.rs CLAUDE.md warning fires when CLAUDE.md imports @AGENTS.md but .claude/CLAUDE.md does not; Claude Code loads both, so warn only when memory files exist and none imports it, and test that case.
target: atelier-sdb0
identity: Independent reviewer (Claude subagent)
commit: cbb151fe4909ae92ee60a07ae384abe9dda975f2
created_at: 2026-10-17T02:41:20.075986456Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: bac5dcc787618df5219c29e488568efa6eed7e13
    recorded_at: 2026-10-17T02:46:55.072101870Z
    resolution_commit: 885d532bb0eb51e68b23acaf0518a0867b94e4c5
    reason: Warn only when no memory file imports @AGENTS.md.
    migrated_from: atelier-ycpw
---

setup.rs CLAUDE.md warning fires when CLAUDE.md imports @AGENTS.md but .claude/CLAUDE.md does not; Claude Code loads both, so warn only when memory files exist and none imports it, and test that case.
