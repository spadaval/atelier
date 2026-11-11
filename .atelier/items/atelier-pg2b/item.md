---
schema: atelier.item
schema_version: 3
id: atelier-pg2b
title: Tell Agent Factory's install procedure how to get the atelier CLI
status: draft
mission: null
labels:
- agent-factory
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-28T02:06:44.271780469Z
---

## Goal

Agent Factory's `skills/agent-factory/procedures/install.md` (in spadaval/.agents, the skill's one copy per ADR 0023) step 1 says to confirm `atelier` is on `PATH` but not how to get it. An agent adopting Atelier in another repository is who the release installer is for.

## Done when

- [ ] The step says to install it with the release installer from Atelier's README when it is missing, and the skill's evaluation (`skills/agent-factory/maintainers/skill-evaluation.md`) is followed for the wording change. The change is committed in spadaval/.agents, not in this repository.

## Context

Found in review of atelier-5tl2 (R2). Skill wording changes behavior, so it needs independent review.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-5tl2 (Name the release installer wherever Atelier tells users how to install or upgrade)
