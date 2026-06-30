---
created_at: "2026-06-30T03:31:02.030382290+00:00"
id: "atelier-0n94"
issue_type: "task"
labels:
- "cli"
- "complexity"
- "dashboard"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ubf2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Resolve work queue identity and fold bucket shortcuts"
updated_at: "2026-06-30T03:31:02.030382290+00:00"
---

## Description

The current work command surface has a junk-drawer risk: `work queue` is being asked to be ready picker, blocked triage, repo-wide operational dashboard, and generic issue inventory. Agent Factory roles do not have a clear normal job for that output, and recent command-audit guidance now treats the surface as unresolved.

## Outcome

The CLI has a clear work-view contract: `atelier work ready` is the small top-level picker, `atelier work blocked` is manager blocker triage, `atelier work active` survives only if it beats `atelier status` for in-flight coordination, `atelier work mission <id>` and `atelier work epic <id>` own scoped dashboards, and `atelier issue list` owns generic inventory. `atelier work queue` is removed, hidden, folded into those views, or redefined with one explicit repo-wide operator question. Help, `atelier man` role guidance, command-audit docs, and tests no longer route normal worker or manager flow through `work queue --ready`, `work queue --blocked`, or repo-wide queue firehoses unless a proven automation-only path remains documented as such.
