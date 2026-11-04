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
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Warn in status when land would fail"
updated_at: "2026-11-04T23:05:33.167989567+00:00"
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

- Whether `--quiet` skips the Handoff line. The cost question is decided in Notes.

## Notes

- 2026-11-04 [decision] status warns using a cheap subset of land's checks (records committed, base commits, owner claims), not the full land run
  Reason: status runs at every session start; the full land took about 0.46 s here and grows with the tracker
  Rejected: Running the full land_text on every status
- 2026-11-04 [decision] The cheap subset covers all four failure kinds in Goal: uncommitted records, unreviewed base commits, done claims that no longer hold (a done child blocked by an open issue), and a branch behind its base
  Reason: Done when needs a test for each kind; the earlier note named the checks loosely and missed the last two
