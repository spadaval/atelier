---
schema: atelier.review
schema_version: 2
id: atelier-0cjk
title: finding on atelier-nz6a @dd46c40d
target: atelier-nz6a
identity: claude-code-reviewer-nz6a (opus-5.5)
commit: dd46c40d9e15b32c55b5af770ed80d89ad7dbafa
created_at: 2026-11-08T23:07:52.558132556Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: orchestrator (opus-5.5)
    commit: 73714c25eb9df3431b02b055561fe5bbbbe4c4e6
    recorded_at: 2026-11-08T23:08:23.312691158Z
    follow_up: atelier-9hkr
    reason: Deferred to backlog follow-up atelier-9hkr under atelier-hu64.
---

low: skills/agent-factory/SKILL.md:8 The opening names only success, "Not an Atelier repository" and atelier not found. Inside an Atelier repository, atelier status can fail another way: a broken .atelier/config.toml gives project_config_parse_error (reproduced, exit 1), and a binary older than the repository's policy refuses too. The text leaves that case to the agent's guess, and a literal reader could step aside in a repository that uses Atelier. The error text names .atelier/, so most agents will carry on. Suggested fix when a line is free: say that any other failure means the repository uses Atelier, so diagnose it and continue.
