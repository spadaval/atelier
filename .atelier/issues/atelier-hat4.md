---
created_at: "2026-11-04T23:37:33.232515927+00:00"
id: "atelier-hat4"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:32.508949707+00:00"
status: "declined"
title: "Say a mission defines no criteria when evidence --criterion targets one without them"
updated_at: "2026-11-11T03:55:32.508949707+00:00"
---

## Goal

`atelier evidence record --criterion` on a mission with no success criteria refuses with "Name the criteria as the mission's Success Criteria bullets spell them (`atelier work mission <id>` lists them)", but that command lists none (crates/atelier-cli/src/commands/evidence.rs ~253). Found in review of atelier-hhce (finding atelier-hknd).

## Done when

The refusal says the mission defines no success criteria and to drop `--criterion`; a CLI test covers it.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes a refusal of `evidence record --criterion`, and the What is removed table removes evidence records and commands.
