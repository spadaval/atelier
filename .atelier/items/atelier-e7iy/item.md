---
schema: atelier.item
schema_version: 3
id: atelier-e7iy
title: Decide whether `status` should offer `merge` on an approved gardener run that waits for its session
status: draft
mission: null
labels:
- redesign
- needs-human
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T18:39:09.297430812Z
---

## Goal

Answer one question about the tool, which is Sumeet's because it touches what the product does: after a gardener's run is approved, `atelier status` ends with `Then: atelier merge`, as it does for any approved item. The procedure says that merge waits for the session with Sumeet. Three of the five first gardeners reported it, and one wrote that an unattended thread would merge.

Both positions:

- **Leave the tool as it is.** `docs/product.md` says no command is reserved for the gardener, and a gardener is the implementer of an item labelled `garden`. `garden.md` step 7 now says `status` offers the merge and that the gardener does not run it before the session (atelier-f6dv). All five first runs stopped correctly, each told to by its task as well.
- **Have the tool know.** A thread that takes the run over, or one started by a session hook with no task, reads only `status`. If it merges, the page closes with Session empty and the done item can no longer record what was agreed. `status` could say, for an approved item labelled `garden`, that its merge waits for the session.

## Done when

- [ ] Sumeet has answered, and the answer is written in this item's Report. If the answer changes the tool, it is filed as an item.

## Scope

A decision. No file changes here.

## Context

Reported by the gardeners of atelier-0019, cosplay-studio-ktgi and astromech-qjle on 2026-11-11. Mission atelier-295u's lead did not change the tool: the specifications name no such behaviour.
