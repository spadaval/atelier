---
created_at: "2026-06-15T03:54:39.128821336+00:00"
id: "atelier-14bv"
issue_type: "task"
labels:
- "docs"
- "state-model"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-larc"
  - kind: "issue"
    id: "atelier-okz2"
  - kind: "issue"
    id: "atelier-q8a1"
  - kind: "issue"
    id: "atelier-wet4"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Define status-derived current work model"
updated_at: "2026-06-15T03:54:39.128821336+00:00"
---

## Description

Update the product and architecture contract so current work in a worktree is derived from canonical Markdown issue status and checkout context. The contract should explicitly remove runtime-only active issue/work association as a source of truth and explain how multiple in_progress issues are rendered without a separate active pointer.

## Outcome

- Product docs and architecture docs define in_progress issues as the current work set for a tracker copy/worktree.
- Docs explain that each Git worktree has its own Markdown tracker copy, so current-work state may diverge by branch/worktree and reconcile through Git.
- Root repair and abandon are classified for removal or replacement because local active-pointer cleanup is no longer a product concept.
- Agent guidance and command audit docs no longer teach durable claim or runtime active-work association as normal workflow.

## Evidence

- File change review shows the new status-derived model in product and
  architecture docs.
- Command audit docs name removed or changed commands and the replacement operator behavior.
- Targeted search transcript shows no remaining guidance that treats runtime work_associations or hidden claim as the current-work source of truth.
- `target/debug/atelier lint atelier-lu10` and `target/debug/atelier export --check` pass.
