---
created_at: "2026-06-16T15:59:44.332777119+00:00"
id: "atelier-em15"
issue_type: "validation"
labels:
- "cli"
- "dependencies"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Validate blocker-aware ordering across work views"
updated_at: "2026-06-16T18:45:37.505786891+00:00"
---

## Description

Independently validate that blocker-aware ordering works as one product behavior across the affected work views. Start from the epic outcomes and command transcripts, not from the implementation summary.

## Outcome

- A representative fixture contains a parent epic, mission-linked work, ready work, blocked work, hidden blockers, and a validation child that depends on implementation work.
- The validation run checks `atelier issue list`, `atelier issue list --ready`, `atelier issue list --blocked`, `atelier issue show`, `atelier graph tree`, `atelier mission status`, `atelier mission list`, and root `atelier status`.
- For each checked surface, the observed output either shows visible blockers before blocked work or records a concrete follow-up issue for a real mismatch.
- For each checked surface, default human rows expose readable state and do not print duplicate category/status tokens.
- Blocked rows provide blocked state and drill-down guidance; exact blocker IDs remain available in the detail surface where repair happens.
- The validation distinguishes acceptable bucket boundaries from ordering bugs, so commands are not forced into one universal mega-view.
- Final evidence is attached to this validation issue and maps each epic Outcome bullet to child proof or a captured transcript.

## Evidence

- Validation transcript records the fixture setup, each command run, and the observed order of relevant issue IDs.
- Validation command transcript records the row-state label shown by each checked command and verifies duplicate category/status tokens are absent from default human rows.
- Evidence record or durable note classifies each surface result against the epic Outcome lines and names follow-up issue IDs for any failures.
- `atelier lint atelier-em15`, `atelier lint atelier-k1ga`, `atelier export --check`, `cargo fmt -- --check`, relevant focused cargo tests, and `git diff --check` pass.

## Notes

The validator should not fix implementation defects while validating unless a new implementation issue is explicitly assigned.
