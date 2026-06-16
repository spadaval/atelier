---
created_at: "2026-06-16T16:18:24.260498038+00:00"
id: "atelier-1xmi"
issue_type: "task"
labels:
- "audit"
- "cli"
- "commands"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-m1r7"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Audit remaining low-level command surfaces"
updated_at: "2026-06-16T16:18:24.260498038+00:00"
---

## Description

Audit adjacent low-level command surfaces so export cleanup does not leave the rest of the command model inconsistent. The goal is a concise product answer for which commands are normal, which are hidden/admin, and which should be changed next.

## Outcome

- `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, `maintenance`, `branch`, and `worktree` are each classified as keep, hide/admin-frame, rename, fold into another command, remove, or defer.
- Any command still shown in root help has a normal operator job and a reason it belongs there.
- Any hidden command has admin/debug/migration framing and does not appear in normal next-action guidance.
- Branch command cleanup is cross-referenced to the branch-lifecycle epic rather than duplicated.
- New follow-up issues are created for command surfaces whose cleanup is not covered by this epic or existing open work.

## Evidence

- Command transcript captures `atelier --help` and representative group help for the audited command families.
- Documentation file diff updates command-audit pages with keep/change/defer classifications and replacement commands.
- Search command transcript over product docs and Agent Factory guidance proves hidden/admin commands are not taught as normal workflow paths.
- Tracker links or issue notes identify follow-up issue IDs for any deferred command cleanup.
- `atelier lint atelier-1xmi`, `atelier doctor`, and `git diff --check` pass.

## Notes

Initial likely candidates: `rebuild` should follow `doctor --fix`; `workflow check` and `diagnostics slow` should stay hidden debug tools; `import-beads` should remain migration-only; `maintenance` should stay explicit danger-zone; `branch` should be demoted by the branch-lifecycle epic; `worktree` likely remains visible because it is real workspace management.
