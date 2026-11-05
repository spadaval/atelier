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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Say a mission defines no criteria when evidence --criterion targets one without them"
updated_at: "2026-11-04T23:37:33.232515927+00:00"
---

## Goal

`atelier evidence record --criterion` on a mission with no success criteria refuses with "Name the criteria as the mission's Success Criteria bullets spell them (`atelier work mission <id>` lists them)", but that command lists none (crates/atelier-cli/src/commands/evidence.rs ~253). Found in review of atelier-hhce (finding atelier-hknd).

## Done when

The refusal says the mission defines no success criteria and to drop `--criterion`; a CLI test covers it.
