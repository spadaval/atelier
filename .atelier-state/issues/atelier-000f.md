---
acceptance: []
blocks:
- "atelier-0006"
- "atelier-000v"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-000p"
- "atelier-000t"
- "atelier-001b"
evidence_required: []
id: "atelier-000f"
issue_type: "task"
labels:
- "assignee:root"
- "feature"
- "spec"
- "validator"
- "workflow"
- "worktree"
links: []
parent: "atelier-000m"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Implement atelier work start/finish/status"
updated_at: "2026-06-10T14:51:26.353744549+00:00"
---

Implement the work-facing lifecycle commands around claims, branch/worktree association, and closeout validators: atelier work start <id>, atelier work finish <id>, and atelier work status. This becomes the normal path for tracked agent work and supersedes scattered timer/session-work usage for ordinary workflow.

Scope includes claim/work association, branch/worktree metadata, dirty-worktree safeguards, configured setup hooks, status reporting, and closeout validator checks that exist at this stage. It does not launch or supervise agents directly.

Acceptance:
Commands support stable JSON where automation needs it; work start records claim, branch, and worktree intent without creating live agent-run rows; work finish checks workflow validators that exist at this stage; status reports current work association; CLI integration tests cover happy path, dirty-worktree rejection, no-agent-launch behavior, and replacement of timer/session-work as the documented normal path.
