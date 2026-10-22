---
created_at: "2026-10-22T23:28:10.364372659+00:00"
id: "atelier-k4xg"
issue_type: "task"
labels:
- "review"
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
status: "todo"
title: "Sync atelier-k8h8's vendored Agent Factory skill edits upstream"
updated_at: "2026-10-22T23:28:10.364372659+00:00"
---

## Goal

Mission atelier-k8h8 edited the vendored Agent Factory skill under .agents/skills/agent-factory/: references/atelier.md (atelier-c3kh's merge default, atelier-yim7's start line, atelier-zuow's record commits and discovered_from, atelier-j8kg's base exemption), references/tracker-records.md (discovered_from) and procedures/orchestrate.md (review request, batching follow-ups). The upstream skill repository spadaval/.agents (head 451c0c9 on 2026-10-22) has none of them, so other repositories using the skill don't get them. Found in review of atelier-j8kg (finding atelier-s4vz).

## Done when

Every change to .agents/skills/agent-factory/ that this mission merged into master is applied to spadaval/.agents and pushed, with the vendored copy and upstream identical for those files (diff shown in the handoff). The push follows that repository's own review rules.
