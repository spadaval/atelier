---
created_at: "2026-10-22T23:28:10.364372659+00:00"
id: "atelier-k4xg"
issue_type: "task"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-k4xg
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "review"
    id: "atelier-s4vz"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T03:50:06.242127168+00:00"
status: "done"
title: "Sync atelier-k8h8's vendored Agent Factory skill edits upstream"
updated_at: "2026-10-23T03:50:06.242127168+00:00"
---

## Goal

Mission atelier-k8h8 edited the vendored Agent Factory skill under .agents/skills/agent-factory/: references/atelier.md (atelier-c3kh's merge default, atelier-yim7's start line, atelier-zuow's record commits and discovered_from, atelier-j8kg's base exemption), references/tracker-records.md (discovered_from) and procedures/orchestrate.md (review request, batching follow-ups). The upstream skill repository spadaval/.agents (head 451c0c9 on 2026-10-22) has none of them, so other repositories using the skill don't get them. Found in review of atelier-j8kg (finding atelier-s4vz).

## Done when

Every change to .agents/skills/agent-factory/ that this mission merged into master is applied to spadaval/.agents and pushed, with the vendored copy and upstream identical for those files (diff shown in the handoff). The push follows that repository's own review rules.

## Close Reason

### Outcome
All of atelier-k8h8's edits to the vendored Agent Factory skill are upstream in spadaval/.agents at 8ec9851: the merge_commit default, start --base, record-commit flags and discovered_from, atelier review request, batching follow-ups under one review, the owner-free trivial exemption on the base, and the one review standard for code and prose with the Constitution amendment. The human approved the push on 2026-10-22.

### Deviations
The Done when asked for the vendored copy and upstream to be identical for the touched files. Five are; four (SKILL.md, procedures/implement.md, references/atelier.md, references/tracker-records.md) still differ by drift that predates this mission and was left alone.

### Decisions
Apply exactly this mission's diff rather than copy the vendored files over upstream, so upstream changes made outside this repository are kept.

### Surprises
The installed skill at ~/.agents/skills/agent-factory is not a git checkout and also differs from the vendored copy.

### Open items
Reconciling the older drift between the vendored copy, upstream and the installed copy. The 'just a doc' evaluation scenario is atelier-3js6.
