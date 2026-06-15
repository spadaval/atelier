---
created_at: "2026-06-15T03:54:02.500039570+00:00"
id: "atelier-lu10"
issue_type: "epic"
labels:
- "cleanup"
- "cli"
- "state-model"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-14bv"
  - kind: "issue"
    id: "atelier-larc"
  - kind: "issue"
    id: "atelier-okz2"
  - kind: "issue"
    id: "atelier-q8a1"
  - kind: "issue"
    id: "atelier-t35w"
  - kind: "issue"
    id: "atelier-wet4"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove active issue and legacy claim systems"
updated_at: "2026-06-15T04:05:54.474930114+00:00"
---

## Description

Atelier should stop maintaining a separate active issue or claim state model outside canonical tracker records. A worktree-local tracker copy can use durable issue workflow status, especially in_progress, to describe current work in that checkout. Runtime cache rebuilds must not erase meaningful current-work state, and legacy claim behavior should be hard-removed.

## Outcome

- Active issue/current work is derived from canonical Markdown state and checkout context, not runtime-only work_associations.
- Legacy hidden claim behavior is removed rather than replaced with a compatibility shim.
- Root lifecycle commands no longer expose cache-repair or local-pointer cleanup concepts.
- Status, man guides, help, docs, and tests teach the simpler status-derived model.
- Agent Factory binding and portable tracker standards no longer teach claim,
  finish, or runtime active-work association as the normal worker flow.

## Evidence

- Child issue proof maps docs, implementation, claim removal, and validation outcomes back to this epic.
- Fresh runtime rebuild transcript proves current-work orientation survives cache deletion because it is derived from Markdown.
- Help and command transcripts prove removed active-pointer and claim surfaces are absent or rejected.
- Agent Factory standards diff and search transcript prove obsolete claim/finish
  worker-flow guidance was removed.
- Closeout review confirms no active work source of truth remains in ignored runtime/cache state.
