---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-0004"
issue_type: "task"
labels:
- "evidence"
- "spec"
priority: "P3"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000u"
  - kind: "issue"
    id: "atelier-001j"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Decide first artifact storage backends for evidence"
updated_at: "2026-06-10T14:51:19.776106093+00:00"
---


Resolve the SPEC.md open question about artifact storage. Specify what evidence artifacts live in the repo, what may live externally, and which backend is first.

## Resolution

TODO: choose the initial artifact storage policy and backend.

## Rationale

Evidence records need stable metadata, size/hash handling, and paths or URIs while avoiding oversized repository history for large artifacts.

## Alternatives Considered

- Store small artifacts directly in the repo.
- Store metadata in repo and large artifacts externally.
- Start with local filesystem paths only.
- Add a configurable external backend immediately.
