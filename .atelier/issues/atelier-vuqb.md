---
created_at: "2026-06-16T16:18:17.780900877+00:00"
id: "atelier-vuqb"
issue_type: "feature"
labels:
- "cli"
- "export"
- "maintenance"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-m1r7"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T17:43:00.545738571+00:00"
status: "done"
title: "Demote or remove export as a public command"
updated_at: "2026-06-16T17:43:00.545738571+00:00"
---

## Description

Remove `atelier export` from the ordinary product surface. Today the command can materialize canonical Markdown from a local projection, which is useful only for migration/debug work and dangerous as a general repair habit.

## Outcome

- `atelier export` is absent from normal root help, role guides, common-command lists, normal recovery hints, validation recipes, and issue Evidence examples.
- If the capability remains, it is hidden or admin-framed with naming and help text that makes the source-of-truth risk explicit.
- Running the retained migration/debug path refuses or loudly warns when it would rewrite tracked canonical records from stale or invalid projection state.
- Removed or unsupported public forms fail clearly and point normal users to `lint`, `doctor`, `doctor --fix`, or the specific record file to edit.
- Tests cover the retained migration/debug path or the removed-command error path, whichever the contract selects.

## Evidence

- Help transcript proves normal root help and role help do not list `atelier export` as a routine command.
- CLI transcript or focused test proves removed or hidden export forms do not rewrite tracked canonical Markdown during ordinary repair.
- CLI transcript or focused test proves the retained admin/debug path rejects stale or invalid projection state before writing tracked records.
- Search command transcript over `AGENTFACTORY.md`, `/root/.agents/skills/agent-factory`, `docs/product`, `docs/architecture/quality`, and `.atelier/issues` proves routine guidance no longer asks agents to run export as handoff proof.
- `atelier lint atelier-vuqb`, `atelier doctor`, focused command-surface tests, and `git diff --check` pass.

## Notes

The legitimate capability is migration/debug rendering from a trusted legacy source. The normal recovery path is never "overwrite Markdown from SQLite."
