---
schema: atelier.item
schema_version: 3
id: atelier-m3hw
title: Run the extension's updater test on Windows
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-16T21:00:22.133067591Z
---

## Goal

Let the updater's test run on the machine the extension is used on. `t3-extension/test-update.mjs` calls `python3` and builds a macOS app as its stand-in for T3 Code, so on Windows it fails at once with `spawn python3 ENOENT`, as it did on 2026-11-16 when the planner's thread ran the 0.1.1 bundle's tests there. The updater itself takes Python from `config.json` or `py` on Windows; only the test is tied to Linux and macOS. Applying an update on Windows has therefore never been run.

## Done when

- [ ] `node t3-extension/run-tests.mjs` passes on Windows with a Desk named by `DESK`, with `test-update.mjs` using the Python the platform has and a stand-in for T3 Code that the Windows installer accepts.
- [ ] The Report shows a run on Windows that installs a newer fixture bundle through `t3-custom update`.

## Scope

In: `t3-extension/`. The Windows run is made over SSH to Sumeet's Windows machine by the thread that starts the item, since no agent on this host runs Windows.
