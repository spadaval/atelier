---
created_at: "2026-06-29T18:20:04.089271885+00:00"
id: "atelier-pc7s"
issue_type: "epic"
labels:
- "cli"
- "dashboard"
- "mission"
- "workflow"
review:
  kind: pull_request
  number: 31
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2uim"
  children:
  - kind: "issue"
    id: "atelier-0n94"
  - kind: "issue"
    id: "atelier-3zcr"
  - kind: "issue"
    id: "atelier-45yt"
  - kind: "issue"
    id: "atelier-brqs"
  - kind: "issue"
    id: "atelier-gsor"
  - kind: "issue"
    id: "atelier-kfey"
  - kind: "issue"
    id: "atelier-p19n"
  - kind: "issue"
    id: "atelier-qo3w"
  - kind: "issue"
    id: "atelier-ubf2"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1mga"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Repair CLI issue listing and mission dashboard"
updated_at: "2026-06-30T03:39:16.314547992+00:00"
---

## Description

Atelier restores a simple generic issue inventory command while keeping bounded operational dashboards under the `work` namespace. `atelier issue list` becomes the obvious all-issues inventory surface, `atelier work ready` and `atelier work blocked` answer small repo-wide bucket questions, and `atelier work mission <mission-id>` becomes a bounded mission dashboard that answers operational questions without proof-gap noise, premature closeout language, empty buckets, or repo-wide queue detours.

## Outcome

The CLI has a clear split: `atelier issue list` lists issue records with useful inventory filters; `atelier work ready`, `atelier work blocked`, and any surviving `atelier work active` answer small repo-wide operational questions; `atelier work mission <mission-id>` shows mission-scoped progress, blocked/ready work, closeout only when relevant, and focused next commands; and `atelier issue transition <id>` defaults to transition names, allowed/blocked state, and failed requirements instead of validator/action debug output. `atelier work queue` is removed, hidden, folded into those views, or redefined around one explicit repo-wide operator question. Help, docs, role guidance, and tests reflect this split and no longer teach contradictory removed-command assumptions.
