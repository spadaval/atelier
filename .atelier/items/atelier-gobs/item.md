---
schema: atelier.item
schema_version: 3
id: atelier-gobs
title: skills/agent-factory/SKILL.md:9 and procedures/install.md:3 For a user who asked to set up Atelier, the opening says to…
status: dropped
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T23:08:23.312691158Z
---

## Goal

Deferred from finding atelier-1yc4 on atelier-nz6a @dd46c40d, by `atelier review defer`.

low: skills/agent-factory/SKILL.md:9 and procedures/install.md:3 For a user who asked to set up Atelier, the opening says to stop using the skill and "offer install". The user has already asked, so the agent should run install, not offer it, and install is part of this skill. Separately, install.md still offers an assess-only path, but outside Atelier the opening stops before it can be reached, and the description no longer names assessment. Agents will most likely do the sensible thing. The opening follows the Done when's own wording. Suggested fix: "if the user asked to set up Atelier, use install; otherwise stop", and either drop assess-only from install.md or allow it in the opening.

## Done when

- [ ] Done when was not specified.

## Links

Relationships of the old record that the new format has no field for.

- parent: atelier-hu64 (Deferred review findings on atelier-nz6a)
- discovered_from review: atelier-1yc4

## Report

Dropped: `procedures/install.md` was deleted when the skill was rewritten (atelier-orb2).
