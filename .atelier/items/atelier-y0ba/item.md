---
schema: atelier.item
schema_version: 3
id: atelier-y0ba
title: Have status say that an approved gardener run's merge waits for its session with Sumeet
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T22:36:55.569602707Z
---

## Goal

Keep a thread that reads only `status` from merging a gardener's run before its session. After a run is approved, `atelier status` ends with `Then: atelier merge`, as for any approved item, while `garden.md` says the merge waits for the session with Sumeet. Three of the five first gardeners reported it. A thread that takes the run over, or one started by a hook with no task, would merge, and the page would close with its Session section empty. Sumeet agreed on 2026-11-13 that the tool should know (atelier-e7iy), with two runs, atelier-0019 and atelier-qb4f, waiting in exactly this state.

## Done when

- [ ] For an approved item labelled `garden` whose Report has an empty Session section, `status` says the merge waits for the session with Sumeet and names what to do: settle the page with him, write the Session section, then merge. It does not end with a bare `Then: atelier merge`.
- [ ] Once the Session section has text, `status` offers the merge as for any approved item.
- [ ] `merge` itself refuses nothing new: this is what `status` says, not a gate.
- [ ] A test covers both states, and `garden.md` step 7 no longer has to warn that `status` offers the merge.

## Scope

In: the status view in `crates/`, its test, and the one step of `skills/agent-factory/procedures/garden.md`.

Out: a command reserved for the gardener; any check in `merge`; the Desk, where atelier-bbia lists such a run in the inbox.

## Context

`docs/product.md` says a gardener is the implementer of an item labelled `garden` and no command is reserved for it. This keeps that: the label and the empty section change one line of what `status` prints.
