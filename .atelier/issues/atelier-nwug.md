---
acceptance: []
created_at: "2026-06-10T16:09:12.915812699+00:00"
evidence_required: []
id: "atelier-nwug"
issue_type: "task"
labels:
- "activity"
- "cli"
- "issue-show"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-pakd"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add recent activity preview and command footer to issue show"
updated_at: "2026-06-10T17:29:23.603761796+00:00"
---

Add a bounded recent activity preview and concise next-command footer to the human `issue show` view.

What:
- Show the newest bounded activity entries for the issue when canonical activity sidecars are available.
- Until sidecars exist, gracefully fall back to existing notes/comments data.
- Default preview limit is 5 entries.
- Do not create a new persistence model for this feature.
- Add a concise footer with next useful commands such as `atelier work start <id>`, `atelier issue comment <id> ...`, and `atelier issue close <id> --reason ...`.
- Choose footer commands based on issue state where practical, while keeping v1 logic simple.

Out of scope:
- Implementing canonical activity sidecars themselves.
- Implementing `atelier history`.
- Adding new command-line flags for preview limits or formats in v1.

Acceptance criteria:
- Human issue show displays up to 5 newest issue activity entries when sidecars are available.
- When sidecars are absent, issue show falls back to current notes/comments without failing.
- Empty activity renders consistently with the rest of the improved view.
- Footer commands are present and appropriate for at least open, in-progress, and closed issue states.
- Focused tests cover fallback behavior when sidecars are absent.

Recommended subskill: agent-factory implement.
