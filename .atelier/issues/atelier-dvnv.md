---
created_at: "2026-10-17T01:10:57.982742072+00:00"
id: "atelier-dvnv"
issue_type: "feature"
labels:
- "cli-output"
- "status"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Warn in status when land would fail"
updated_at: "2026-10-17T01:10:57.982742072+00:00"
---

## Goal

Status is becoming the SessionStart view (atelier-sdb0), but it does not report failures that `land` (the Stop hook) will raise. In a scratch clone, status said nothing in any of these states:
- master 1 ahead with an unreviewed code commit (`land_base_unreviewed`);
- an owner branch with a done child blocked by an open issue (`land_invariants_unmet`);
- uncommitted records after `atelier start` (`land_records_uncommitted`);
- an owner branch 1 behind its base.

Status showed `[ahead 1]` or a dirty path and suggested ready work. The agent learns about the failure only when the Stop hook blocks at the end of the turn.

## Done when

- Status has a `Handoff` line: `✓ land passes`, or `⚠ land fails: atelier-jb2t done but blocked by atelier-ihns (atelier land for details)`. Only the first failure is shown, with a count of the rest.
- Tests cover the pass case and one failure of each kind above.

## Unknowns

- Cost. Land took about 0.46 s on this repository after atelier-37uq. Decide whether status runs the full `land_text` (crates/atelier-cli/src/commands/session.rs:297) or a cheap subset (records committed, base commits, owner claims), and whether `--quiet` skips it.
