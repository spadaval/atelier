---
created_at: "2026-10-17T19:00:36.340066193+00:00"
id: "atelier-u4ql"
issue_type: "feature"
labels:
- "cross-cutting"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Say reinstall atelier when records need a newer binary"
updated_at: "2026-10-17T19:00:36.340066193+00:00"
---

## Goal

After o6h9 migrated records, the installed binary failed to parse them and the Stop hook's land failed on master with a parse error, not an instruction (b32c39e6).

## Done when

When a record's schema version or field is newer than the running binary supports, every command reports 'records need atelier >= X; reinstall' with the install command, instead of a parse error. A test with a future schema version covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
