---
schema: atelier.item
schema_version: 3
id: atelier-9hkr
title: skills/agent-factory/SKILL.md:8 The opening names only success, "Not an Atelier repository" and atelier not found. Insi…
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

Deferred from finding atelier-0cjk on atelier-nz6a @dd46c40d, by `atelier review defer`.

low: skills/agent-factory/SKILL.md:8 The opening names only success, "Not an Atelier repository" and atelier not found. Inside an Atelier repository, atelier status can fail another way: a broken .atelier/config.toml gives project_config_parse_error (reproduced, exit 1), and a binary older than the repository's policy refuses too. The text leaves that case to the agent's guess, and a literal reader could step aside in a repository that uses Atelier. The error text names .atelier/, so most agents will carry on. Suggested fix when a line is free: say that any other failure means the repository uses Atelier, so diagnose it and continue.

## Done when

- [ ] Done when was not specified.

## Links

Relationships of the old record that the new format has no field for.

- parent: atelier-hu64 (Deferred review findings on atelier-nz6a)
- discovered_from review: atelier-0cjk

## Report

Dropped: Finding on the old SKILL.md opening, rewritten by atelier-orb2; the new opening steps aside when `status` fails for want of a Git checkout or an Atelier config.
