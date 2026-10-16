---
created_at: "2026-10-16T23:52:05.622714060+00:00"
id: "atelier-mp6v"
issue_type: "feature"
labels:
- "orientation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Stop the Stop hook pushing commits on master for uncommitted records alone"
updated_at: "2026-10-16T23:52:05.622714060+00:00"
---

## Goal

`atelier land --hook claude` runs at the end of every Claude Code turn. On the base branch with one uncommitted `.atelier/` record and no other problem it returns `decision: block` ("commit canonical records before continuing"), which pushes the agent to commit on master without being asked. Found while installing the hooks in this repository (review finding atelier-4cid on atelier-bw0b).

## Done when

On the base branch, when the only failed invariant is uncommitted canonical records, the Stop hook returns a warning (exit 0, no block) naming the paths, while plain `atelier land` still fails. Every other failure still blocks. A hook test covers both cases, and `docs/product/sessions-and-hooks.md` describes the rule.

## Context

`crates/atelier-cli/src/commands/session.rs` (land hook adapter). Hook cost per turn (about 5.8 s here) is being addressed by atelier-37uq.

## Unknowns

Whether the same relaxation should apply on work branches, where uncommitted records usually belong with the work in progress.
