---
created_at: "2026-10-15T23:47:14.113280867+00:00"
id: "atelier-92mz"
issue_type: "task"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Resync the Agent Factory skill with the adoption changes"
updated_at: "2026-10-15T23:47:14.113280867+00:00"
---

## Goal

ADR 0020 says a change that invalidates the skill's Atelier map updates the upstream skill in the same piece of work. The adoption fixes change several things the vendored skill states: epics become parent issues (atelier-o6h9), `issue update --body` works on every type (atelier-kaor), bundles say `blocked_by` (atelier-kaor), the backlog state and `create --status` (atelier-ha6k), record-only commits on the base (atelier-3mjv), the mission strategy link (atelier-9m4k), and needs-human work (atelier-q8ok).

## Done when

The upstream skill in spadaval/.agents describes the merged behaviour, `.agents/skills/agent-factory` is resynced verbatim, and `diff -r` against upstream is empty.

## Context

Filed 2026-10-15 while implementing the adoption records. The vendored copy must not be edited locally.
