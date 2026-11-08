---
created_at: "2026-11-08T05:52:15.329120217+00:00"
id: "atelier-9s9i"
issue_type: "task"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-dkul"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-g6rp"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Fold codex-reflect's harness-neutral retrospective method into audit.md before it is deleted"
updated_at: "2026-11-08T21:02:48.188728949+00:00"
---

## Goal

`codex-reflect` in spadaval/.agents builds retrospectives from Codex session logs with Artifact Hub, which is being deleted. Keep the parts that apply to any harness in Agent Factory's `audit.md`, so retiring it loses nothing.

## Done when

- [ ] `skills/agent-factory/procedures/audit.md` gains at most about 15 lines with what applies beyond Codex: start from raw transcripts rather than memory, verify the transcript around each failure and handoff claim, follow subagent and thread lineage to the root, and how to report a session retrospective (what went well, what went wrong, what to change in code, tooling, guidance or skills). Codex paths, Artifact Hub and the report app are not carried over.
- [ ] The Report lists what was left out and why.
- [ ] The skill stays within `atelier-qj9d`'s line limits.

## Scope

`skills/agent-factory/procedures/`.

## Context

The gardener (atelier-3sk9) reads transcripts too and comes after this item; it moves or links this method, so this item does not wait for it.
