---
schema: atelier.item
schema_version: 3
id: atelier-a9jo
title: The cutover's agent template still bans every remote branch deletion
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T10:17:47.705738316Z
---

## Goal

`scripts/cutover/agents/atelier.md`, the `AGENTS.md` passage the cutover
gives a converted repository, still says never to delete remote branches
without the human. Since `atelier-gqtw`, `atelier merge` deletes the branch
it merged, which Sumeet approved on 2026-11-22, and this repository's
`AGENTS.md` says so. A repository converted from the template would tell
its agents the tool breaks its own rule.

## Done when

- [ ] The template's rule matches `AGENTS.md`: never force-push the base, and never delete a remote branch by hand; `atelier merge` alone deletes the branch it merged.
- [ ] `scripts/cutover/README.md` and the kit's tests still pass.

## Scope

In: `scripts/cutover/agents/atelier.md` and the kit's tests if they compare it.
