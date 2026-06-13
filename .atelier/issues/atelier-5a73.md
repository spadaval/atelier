---
created_at: "2026-06-13T21:58:03.428554959+00:00"
id: "atelier-5a73"
issue_type: "task"
labels:
- "artifact-update"
- "cli"
- "stabilization"
- "workflow"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Clarify workflow status filters and category aliases"
updated_at: "2026-06-13T21:58:03.428554959+00:00"
---

## Description

Workflow status is repository-owned, while status category is derived
orientation metadata. The issue-list filter layer still mixes exact workflow
status names with built-in category aliases such as `active` and
`in_progress`, so operators cannot tell which grammar is public contract and
which is convenience orientation.

## Outcome

- Product CLI contract defines whether `--status` accepts exact workflow
  statuses, derived categories, or separate flags for each concept.
- Hardcoded aliases that leak default workflow vocabulary are removed,
  renamed, or documented as explicit compatibility choices.
- Help text, Agent Factory guidance, and issue-list/status behavior agree on
  the chosen grammar.

## Evidence

- Artifact or docs diff records the status-filter grammar and category-alias
  policy.
- Help transcript proves `atelier issue list --help` describes the chosen
  status/category surface accurately.
- Focused CLI tests or transcripts cover exact workflow status filtering and
  category filtering, including at least one rejected ambiguous value if the
  contract requires rejection.
- `atelier lint atelier-5a73` and `atelier export --check` pass.

## Notes

Audit evidence: ADR 0005 separates workflow status from status category, but
`src/commands/agent_factory.rs` parses hardcoded aliases before exact status
matching and maps category `active` back to `in_progress` for display.
`src/main.rs` help and `AGENTFACTORY.md` still teach default status names.
