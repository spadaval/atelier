---
created_at: "2026-11-09T02:02:36.297141137+00:00"
id: "atelier-7o4r"
issue_type: "task"
labels:
- "forge"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
  - kind: "issue"
    id: "atelier-fad5"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-vqoy"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Run this repository on the one-command merge: retire the hand-run routine from AGENTS.md and the skill"
updated_at: "2026-11-09T02:02:36.297141137+00:00"
---

## Goal

Once `merge --push` drives the forge, rewrite `AGENTS.md`'s Merge Policy and the skill's `references/atelier.md` so agents run `atelier merge <id> --push` instead of pushing the branch, opening a pull request, waiting for CI and pushing the base by hand, and show the mission's one-command-merge criterion on a real item.

## Done when

- [ ] `AGENTS.md`'s Merge Policy and `skills/agent-factory/references/atelier.md` name `atelier merge <id> --push` as the way work reaches the base, and neither tells agents to run `gh pr create`, wait on checks or push the base by hand.
- [ ] One real item in this repository merges with `atelier merge <id> --push` alone while another commit reaches master after its CI run; the Report links the transcript and the merged pull request.

## Scope

Provisional until atelier-fad5 and the pull-request item land. Moving astromech's policy is separate backlog.
