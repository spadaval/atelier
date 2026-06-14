---
created_at: "2026-06-14T02:48:37.165982715+00:00"
id: "atelier-4p7q"
issue_type: "epic"
labels:
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-72ct"
  - kind: "issue"
    id: "atelier-isd5"
  - kind: "issue"
    id: "atelier-zah8"
  children:
  - kind: "issue"
    id: "atelier-4yrt"
  - kind: "issue"
    id: "atelier-7sbv"
  - kind: "issue"
    id: "atelier-a85s"
  - kind: "issue"
    id: "atelier-gh3m"
  - kind: "issue"
    id: "atelier-grmn"
  - kind: "issue"
    id: "atelier-gsq1"
  - kind: "issue"
    id: "atelier-liqk"
  - kind: "issue"
    id: "atelier-od8a"
  - kind: "issue"
    id: "atelier-papa"
  - kind: "issue"
    id: "atelier-q5v0"
  - kind: "issue"
    id: "atelier-qnxs"
  - kind: "issue"
    id: "atelier-sxh8"
  - kind: "issue"
    id: "atelier-vau5"
  - kind: "issue"
    id: "atelier-vj08"
  - kind: "issue"
    id: "atelier-xbr0"
  - kind: "issue"
    id: "atelier-ywqa"
  - kind: "issue"
    id: "atelier-yyx4"
  - kind: "issue"
    id: "atelier-zrqa"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T08:23:32.579730444+00:00"
status: "done"
title: "Epic: Make CLI errors corrective for common agent mistakes"
updated_at: "2026-06-14T08:23:32.579730444+00:00"
---

## Description

Atelier CLI errors and help guide agents away from wrong record-kind commands,
removed command names, invalid evidence relation vocabulary, generic
relationship surfaces, low-level state diagnostics, and obsolete integration
commands.

## Outcome

Common mistakes produce corrective messages with the supported next command
instead of generic usage churn, and root help centers mission/proof work rather
than legacy or implementation-mechanic command families.

## Evidence

Focused CLI tests cover wrong-kind IDs, removed/likely command names, evidence
attach relation errors, removed generic link/dep/integrations surfaces, hidden
legacy issue command removal, and root help hierarchy.
