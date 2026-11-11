---
schema: atelier.item
schema_version: 3
id: atelier-qxmr
title: Point the Desk at the live repositories once this host's cutover has run
status: draft
mission: null
labels:
- loose_end
priority: medium
blocked_by:
- atelier-wd1i
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T02:30:20.131694053Z
---

## Goal

After this host's cutover (atelier-wd1i), change the installed Desk service's roots from the converted scratch clones to the five live repositories, and check a mission page of each against `atelier show` there.

## Done when

- [ ] The service's configuration names the live repositories, it has been restarted, and for one open mission in each repository the page's title, status and item counts match `atelier show <mission>`.

## Context

Outside mission atelier-z4cr on purpose, so that mission can close before the cutover. Strategy: `docs/strategies/desk.md`, Derisking.
