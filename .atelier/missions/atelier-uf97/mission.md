---
schema: atelier.mission
schema_version: 3
id: atelier-uf97
title: 'Let agents write, check and repair tracker records: one at a time, by section or as a bundle'
status: dropped
labels:
- cli-output
lead: null
ready: null
created_at: 2026-10-17T01:44:17.636200248Z
---

## Outcome

An agent can create and change tracker records without shell quoting: one at a time, one section at a time, or many at once as a bundle. Each record-writing command confirms exactly what it wrote and the next step, and `bundle preview` refuses exactly what `bundle apply` refuses; refusing an invalid record before it is written is atelier-ilba's, done. `atelier check` applies one rule set however it is called, reports each fault once against the record to fix, says what it checked, and gives a fault that arrives by merge or hand edit a repair route that works while the graph does not load.

## Success criteria

- refuse-then-confirm: for `issue create`, `update`, `note`, `link`, `unlink`, `transition` and `bundle apply`, a table-driven CLI test shows that valid input prints the values written, and an integration test shows that `bundle preview` refuses exactly the inputs `bundle apply` refuses, writing nothing.

## Result

Dropped: Its items described issue create, update, link, transition, check <id> and bundle, all removed in 0.5; each is dropped with its own reason. The one fault that survives the rewrite is atelier-k55o, now outside a mission.
