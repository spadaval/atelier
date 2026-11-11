---
created_at: "2026-11-11T02:17:27.158256540+00:00"
id: "atelier-va99"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-e05n"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-h3hl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Install the Desk as a service on this host, on converted clones, and check every mission page against the CLI"
updated_at: "2026-11-11T02:17:27.158256540+00:00"
---

## Goal

Preliminary. The Desk runs as a user service on this host, reached over the tailnet, serving converted scratch clones of all five repositories. It does not wait for the cutover: pointing the service at the live repositories is the loose end atelier-qxmr.

## Done when

- [ ] Provisional: the service starts at boot and after a crash; the strategy's desk-matches-cli criterion is shown by a validation run on converted scratch clones of all five repositories, made with the converter at the commit the service is built from; `docs/architecture/desk.md` says how to install and update it.
