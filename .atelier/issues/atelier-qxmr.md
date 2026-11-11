---
created_at: "2026-11-11T02:30:20.131694053+00:00"
id: "atelier-qxmr"
issue_type: "loose_end"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-oc9k"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-va99"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-wd1i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Point the Desk at the live repositories once this host's cutover has run"
updated_at: "2026-11-11T02:30:20.131694053+00:00"
---

## Goal

After this host's cutover (atelier-wd1i), change the installed Desk service's roots from the converted scratch clones to the five live repositories, and check a mission page of each against `atelier show` there.

## Done when

- [ ] The service's configuration names the live repositories, it has been restarted, and for one open mission in each repository the page's title, status and item counts match `atelier show <mission>`.

## Context

Outside mission atelier-z4cr on purpose, so that mission can close before the cutover. Strategy: `docs/strategies/desk.md`, Derisking.
