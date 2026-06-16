---
created_at: "2026-06-16T16:17:58.297737981+00:00"
id: "atelier-c4b8"
issue_type: "epic"
labels:
- "cli"
- "commands"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-1xmi"
  - kind: "issue"
    id: "atelier-2sut"
  - kind: "issue"
    id: "atelier-a7gd"
  - kind: "issue"
    id: "atelier-jezn"
  - kind: "issue"
    id: "atelier-m1r7"
  - kind: "issue"
    id: "atelier-vuqb"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T17:58:39.876764341+00:00"
status: "done"
title: "Epic: Clean up low-level command surfaces"
updated_at: "2026-06-16T17:58:39.876764341+00:00"
---

## Description

Atelier still exposes and teaches some low-level command surfaces that are implementation mechanics rather than product workflows. The worst example is `atelier export`: it can rewrite tracked canonical Markdown from a derived projection even though the target architecture says Markdown is the source of truth and projections are rebuildable.

Clean up the command surface so normal agents use product commands: `status`, `mission status`, `issue show/list/transition/close`, `evidence record`, `lint`, and `doctor`. Low-level migration, projection, telemetry, and destructive-maintenance commands should be hidden, admin-framed, renamed, or removed according to a documented command boundary.

## Outcome

- Target-state docs define the boundary between normal workflow commands, admin maintenance commands, hidden debug diagnostics, and temporary migration commands.
- `atelier export` is no longer a normal operator command, handoff gate, validation recipe, or generic repair step.
- Projection repair is routed through `atelier doctor` and `atelier doctor --fix`, with repair output that never suggests tracked canonical records should be regenerated from local runtime state during normal work.
- `atelier rebuild` is either hidden or folded into the same local-projection repair story; normal users are not asked to run it as routine tracker hygiene.
- Agent Factory guidance, role guides, command audit docs, and issue Evidence boilerplate stop teaching low-level export/rebuild checks as normal completion proof.
- Adjacent low-level commands are audited and either confirmed as intentionally hidden/admin-only or queued for cleanup.

## Evidence

- Documentation file diff maps normal, admin, debug, and migration command categories to concrete command families.
- Help transcript proves root and role-oriented help omit low-level export/rebuild workflows from normal next actions.
- Search command transcript over product docs, Agent Factory guidance, command audits, and tracker issue templates proves normal guidance no longer teaches export/rebuild checks as routine handoff proof.
- Command transcript or focused tests prove stale projection repair routes through `doctor` or `doctor --fix` without rewriting tracked canonical Markdown from runtime state.
- Integrated validation evidence classifies each adjacent command family as kept, hidden/admin-only, renamed, removed, or deferred with a follow-up issue.
- `atelier lint`, `atelier doctor`, focused command-surface tests, and `git diff --check` pass.

## Notes

This does not remove legitimate migration/debug capability by accident. It changes which surfaces are product workflows and which surfaces are explicit admin/debug tools.
