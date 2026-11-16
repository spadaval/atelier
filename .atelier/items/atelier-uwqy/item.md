---
schema: atelier.item
schema_version: 3
id: atelier-uwqy
title: Read the fixture Desk's address only once it is printed whole
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-16T21:00:22.097514011Z
---

## Goal

Stop a local run of the extension's tests from failing before the browser opens. `t3-extension/fixture-desk.mjs` sometimes reads the Desk's address from its output before the line is printed whole. A reviewer of atelier-wlyn found it in round 2 (`.atelier/items/atelier-wlyn/reviews/2-vduv.md`); the code is older than that item.

## Done when

- [ ] The fixture reads the address only from a complete line, and a test or a repeat run of 50 starts shows no failure.

## Scope

In: `t3-extension/fixture-desk.mjs` and its tests.
