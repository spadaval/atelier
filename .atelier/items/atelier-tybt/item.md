---
schema: atelier.item
schema_version: 3
id: atelier-tybt
title: Show a finding that waits on the lead's ruling on the Desk's Disputed page
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T07:54:41.070090557Z
---

## Goal

Since atelier-hlsw an implementer can answer a finding that blocks with
`<finding>: not built: <reason>`, which takes it to the mission's lead.
`status`, `show <mission>` and `list --disputed` list such a finding as
waiting on the lead's ruling. The Desk's Disputed page does not, so the
human sees it nowhere on the Desk. atelier-hlsw left it out because the
Desk was outside its Scope.

## Done when

- [ ] The Desk's Disputed page lists each finding answered `not built` and
  not yet ruled on, with its reason and the item, as `list --disputed`
  does.
- [ ] A test covers a fixture item with one.

## Scope

`crates/atelier-desk` and the Desk's app under `desk/`.
