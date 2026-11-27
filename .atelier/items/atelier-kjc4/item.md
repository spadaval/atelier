---
schema: atelier.item
schema_version: 3
id: atelier-kjc4
title: t3-extension's test-extension.mjs fails intermittently in CI
status: draft
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T07:51:21.450058240Z
---

## Goal

`node t3-extension/run-tests.mjs` fails now and then with no change to what
it tests. It failed `master`'s CI on `70a2e2801` (the merge of
`atelier-1n49`, which changed no extension code) with `Error: thread-b
scanned timed out`
(https://github.com/spadaval/atelier/actions/runs/44997273737), and failed
twice for `atelier-js4a`'s round 1 reviewer with another error. A run that
fails this way makes `atelier merge` refuse a sound change until it is run
again. It waits until Sumeet reopens `t3-extension/`, which he froze on
2026-11-22.

## Done when

- [ ] The cause of the `thread-b scanned` timeout is found and fixed, not hidden by a retry or a longer wait.
- [ ] `node t3-extension/run-tests.mjs` passes 20 runs in a row in CI's container or on a host like it.

## Scope

In: `t3-extension/` tests and the code they find at fault.
