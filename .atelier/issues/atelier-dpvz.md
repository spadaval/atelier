---
created_at: "2026-10-17T09:06:42.943158161+00:00"
id: "atelier-dpvz"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Check the kind of every bundle reference before apply"
updated_at: "2026-10-17T09:06:42.943158161+00:00"
---

## Goal

Found in review of atelier-lr8o:
- A short ID matching both an issue and an evidence record resolves to the issue silently, where a clash within one kind is an ambiguity error (`crates/atelier-app/src/record_index.rs`, about lines 993-997; finding atelier-asxi). Only reachable across project slugs.
- An evidence ID in an issue's `blocked_by` or `blocks` passes preview and fails at apply with "blocks relationship endpoints must both be issues", not the wrong-kind message (`bundle.rs`, about line 1347; finding atelier-jqb7).
- An update entry's own `id` still says "Update u names missing issue zzzz", including for an evidence ID (`bundle.rs`, about lines 125-127; finding atelier-eoxo).

## Done when

- A cross-kind short-ID clash is reported as ambiguous.
- Preview refuses a wrong-kind reference in every slot with the shared wrong-kind message.
- Update entry IDs use the shared not-found and wrong-kind messages.
- Bundle tests cover each case.
