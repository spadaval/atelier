---
created_at: "2026-06-12T01:41:31.009321638+00:00"
id: "atelier-9jbu"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "ergonomics"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Reduce issue subcommands to lifecycle essentials"
updated_at: "2026-06-12T03:35:44.546255361+00:00"
---

## Description

Redesign the `atelier issue` command group so it exposes only ordinary issue
lifecycle operations. The current surface mixes lifecycle, dependency graph,
search, notes, destructive maintenance, validation markers, and hierarchy views,
which makes the command group hard for agents to scan and explain.

Target primary surface:

- `atelier issue create`
- `atelier issue list`
- `atelier issue show`
- `atelier issue update`
- `atelier issue close`

Fold redundant issue commands into lifecycle commands where the operation is
still issue-local:

- `issue quick` -> `issue create --work`
- `issue subissue` -> `issue create --parent <id>`
- `issue reopen` -> `issue update <id> --status open`
- `issue label` -> `issue update <id> --label <label>`
- `issue unlabel` -> `issue update <id> --remove-label <label>`
- `issue blocked` -> `issue list --blocked`

Move non-lifecycle capabilities to better command homes:

- `issue block` / `issue unblock` -> dependency command group
- `issue relate` / `issue unrelate` / `issue related` -> typed link command group
- `issue impact` / `issue tree` -> graph or hierarchy views
- `issue search` -> cross-record search
- `issue comment` -> note or activity command group
- `issue delete` -> maintenance/destructive command group

Remove commands that are too broad or misplaced unless the audit proves a
specific replacement is required:

- `issue close-all`
- `issue next`
- `issue tested`

## Outcome

- The desired issue command surface is documented with keep/fold/move/remove
  classifications for every current issue subcommand.
- The reduced `atelier issue --help` target is limited to lifecycle essentials.
- Old workflows have explicit new homes or are intentionally removed with a
  rationale.
- Agent Factory guidance uses the reduced command set.
- Transcript tests prove the replacement commands cover create, parented
  create, active-work create, reopen, label/unlabel, blocked-list, dependency,
  link, hierarchy, search, notes, and deletion workflows where retained.

## Evidence

Evidence was not specified in the legacy issue record.
