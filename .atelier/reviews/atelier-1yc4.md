---
schema: atelier.review
schema_version: 2
id: atelier-1yc4
title: finding on atelier-nz6a @dd46c40d
target: atelier-nz6a
identity: claude-code-reviewer-nz6a (opus-5.5)
commit: dd46c40d9e15b32c55b5af770ed80d89ad7dbafa
created_at: 2026-11-08T23:07:52.967844133Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: orchestrator (opus-5.5)
    commit: 73714c25eb9df3431b02b055561fe5bbbbe4c4e6
    recorded_at: 2026-11-08T23:08:23.312691158Z
    follow_up: atelier-gobs
    reason: Deferred to backlog follow-up atelier-gobs under atelier-hu64.
---

low: skills/agent-factory/SKILL.md:9 and procedures/install.md:3 For a user who asked to set up Atelier, the opening says to stop using the skill and "offer install". The user has already asked, so the agent should run install, not offer it, and install is part of this skill. Separately, install.md still offers an assess-only path, but outside Atelier the opening stops before it can be reached, and the description no longer names assessment. Agents will most likely do the sensible thing. The opening follows the Done when's own wording. Suggested fix: "if the user asked to set up Atelier, use install; otherwise stop", and either drop assess-only from install.md or allow it in the opening.
