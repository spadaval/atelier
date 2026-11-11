---
created_at: "2026-11-09T00:45:59.871807783+00:00"
id: "atelier-wpj2"
issue_type: "feature"
labels:
- "agent-attribution"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-7yy9"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-aab1"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-bnek"
    type: "advances"
  - kind: "issue"
    id: "atelier-zpmb"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T02:18:52.844475956+00:00"
status: "superseded"
title: "Show each item's T3 threads, roles, models and live status on the site"
updated_at: "2026-11-11T02:18:52.844475956+00:00"
---

## Goal

Show who is working on what: the site's item, repository and inbox pages show each item's T3 threads with role, harness, provider, model and effort, live status (running, waiting on the human, done, failed), and a link into T3.

## Done when

- [ ] Provisional: an item's page lists each thread recorded for it with role, provider, model, effort and live status, and a working link into T3; a Grok thread is resolved from its session ID.
- [ ] Provisional: the inbox shows threads waiting on the human beside the items waiting on him.

## Unknowns

- How to read T3; settled by the T3 read spike. Make this item ready after the spike, the serve item and the attribution item land.

## Report

Folded into the Desk mission atelier-z4cr: its item atelier-e05n shows each item's threads from T3 Code's MCP server (ADR 0032)
