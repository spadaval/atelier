---
schema: atelier.item
schema_version: 3
id: atelier-taxm
title: Use one function to detect a skill linked to a checkout, in setup --link and the update check
status: draft
mission: null
labels:
- agent-factory
- setup
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T07:07:49.807640842Z
---

## Goal

`atelier-45gr` (`freshness::linked_checkout`) and `atelier-8bj8` (`setup --link`) each detect whether the installed agent-factory skill is a symlink into a checkout. Two copies of the rule can drift, so a skill one reports as linked the other could treat as a copy.

## Done when

- [ ] One function decides whether `~/.agents/skills/agent-factory` or `~/.claude/skills/agent-factory` is linked to a checkout and which one, and both `setup --link`/`--unlink` and the update check call it. Existing tests for both still pass.
