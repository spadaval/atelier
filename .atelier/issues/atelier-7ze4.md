---
created_at: "2026-06-23T15:20:59.633717961+00:00"
id: "atelier-7ze4"
issue_type: "task"
labels:
- "artifact-update"
- "docs"
- "human-output"
- "workflow-state"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-5sgx"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-23T22:37:18.744519500+00:00"
status: "done"
title: "Define shared human CLI output grammar and formatter contract"
updated_at: "2026-06-23T22:37:18.744519500+00:00"
---

## Description

Decide and document the shared default human output grammar before implementation slices rewrite command rendering. The design must absorb the actual agent complaint audit, especially work-selection, blocker, lifecycle, and help failures that are not solved by visual formatting alone.

## Outcome

- `docs/product/human-cli-output.md` and `docs/product/command-audit/human-output-refresh.md` agree on command titles, summaries, rows, grouped sections, bounded lists, footers, quiet output boundaries, and color policy.
- `docs/product/human-cli-output.md`, `docs/product/command-audit/human-output-refresh.md`, and `docs/product/command-audit/agent-complaints.md` agree on the vocabulary for workflow state, blocker state, display role, next commands, and public recovery guidance.
- The grammar distinguishes executable work, selectable work, blocked work, blocked-through-parent work, context-only rows, and omitted rows without exposing internal storage/index details in normal output.
- The grammar states how retired, hidden, admin, and replacement commands appear in help/errors so duplicate lifecycle paths and implementation-shaped names are not reintroduced as normal workflow.
- The design explicitly references the Zen principles: repository state remains authoritative, proof stands on its own, output models domain concepts instead of flattening them, coordination is visible, every formatting feature justifies cost, and obsolete output patterns are removed once replaced.
- The contract names which behavior belongs in shared formatter helpers, which state-correctness checks belong to command/app logic, and which behavior stays command-specific.

## Evidence

- File changes in `docs/product/human-cli-output.md`, `docs/product/command-audit/human-output-refresh.md`, and `docs/product/command-audit/agent-complaints.md` show the shared grammar, workflow/blocker/display-role vocabulary, color behavior, list budget rules, and footer/drill-down rules.
- The design explicitly maps the actual complaint themes to the child implementation issues so status correctness bugs, hidden ready work, parent-blocker ambiguity, command-language, and stale-help failures have owners.
- `git diff --check -- docs/product docs/architecture` passes.
- `target/debug/atelier lint atelier-c0qc` passes.
