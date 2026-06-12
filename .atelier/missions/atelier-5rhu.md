---
created_at: "2026-06-12T00:57:36.372898065+00:00"
id: "atelier-5rhu"
data: "{\"constraints\":[\"Keep human-readable CLI output as the primary agent interface; do not restore command-result JSON.\",\"Treat workflow validators as guardrails and clarity features as signposts.\",\"Preserve canonical .atelier/ records and rebuildable projection boundaries.\",\"Prefer compatibility aliases or staged deprecation for moved commands unless an issue explicitly proves removal is safe.\",\"Keep scope to CLI clarity, command organization, output usefulness, docs, and validation evidence.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Broad polish work can become unbounded without an audit and explicit command inventory.\",\"Renaming or moving commands can break Agent Factory guidance and existing operator habits.\",\"Better output can regress quiet/script-friendly acknowledgements if not tested separately.\"],\"validation\":[\"Representative command transcripts prove clearer status, history, mission, issue transition options, start/finish, worktree, help, and next-action output.\",\"Docs and Agent Factory guidance match the final command hierarchy.\",\"cargo fmt -- --check, focused CLI integration tests, cargo nextest run, atelier export --check, atelier lint, atelier doctor, and mission closeout readiness checks pass before closeout.\",\"Closeout proof may use internal workflow checks, but user-facing guidance should frame them through status, issue transition options, start/finish, and mission closeout readiness rather than requiring operators to call a workflow-validator subsystem directly.\"],\"work\":[]}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0858"
    role: "advances"
  - kind: "issue"
    id: "atelier-4wif"
    role: "advances"
  - kind: "issue"
    id: "atelier-6dvg"
    role: "advances"
  - kind: "issue"
    id: "atelier-m065"
    role: "advances"
  - kind: "issue"
    id: "atelier-o2a4"
    role: "advances"
  - kind: "issue"
    id: "atelier-vr9g"
    role: "advances"
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "open"
title: "Clarify and streamline agent CLI workflows"
updated_at: "2026-06-12T02:53:38.257798145+00:00"
---

Refine Atelier CLI so agents can quickly understand current state, recent activity, next work, workflow guardrails, and command organization without relying on private context or generic next-command output. Treat closed mission atelier-8bky and decision atelier-t24d as historical context, not reopened active work. Recommended subskill: agent-factory orchestrate.
