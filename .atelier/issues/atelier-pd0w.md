---
acceptance: []
created_at: "2026-06-10T16:08:34.447073812+00:00"
evidence_required: []
id: "atelier-pd0w"
issue_type: "epic"
labels:
- "agent-factory"
- "assignee:root"
- "cli"
- "human-output"
- "issue-show"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-nwug"
  - kind: "issue"
    id: "atelier-p1yj"
  - kind: "issue"
    id: "atelier-pakd"
  - kind: "issue"
    id: "atelier-vxte"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Epic: Better issue show human view"
updated_at: "2026-06-10T17:43:52.159841537+00:00"
---

Upgrade the default human output for `atelier issue show <id>` into a richer, scan-friendly single-issue view inspired by `jira work --format details`.

Scope:
- Enhance `commands::agent_factory::show` human output only.
- Keep `atelier issue show <id> --json` stable and unchanged for agents and scripts.
- Ensure `atelier show <id>` inherits the improved output through the existing shortcut path.
- Make the single-issue view useful for humans and orchestrators by showing status, hierarchy, blockers, child progress, ownership, recent activity, and next useful commands.

Out of scope:
- New `issue view` command.
- New `--format` flag.
- Grouped work queue views.
- Mission Control TUI work.
- New persistence model for activity.

Acceptance criteria:
- Child issues cover core rendering, hierarchy/dependency context, recent activity/footer behavior, and regression validation.
- JSON compatibility remains the explicit boundary for scripts and agents.
- Future implementers can execute each child without needing the original chat plan.

Recommended subskill: agent-factory orchestrate.
