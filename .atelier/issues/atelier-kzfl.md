---
created_at: "2026-06-16T15:59:40.138266627+00:00"
id: "atelier-kzfl"
issue_type: "feature"
labels:
- "cli"
- "dependencies"
- "mission"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-em15"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Align mission list actionability with blocker-aware work"
updated_at: "2026-06-16T18:39:54.671671601+00:00"
---

## Description

Align mission list actionability with blocker-aware work selection and readable state display. `atelier mission list` should stay a compact mission overview, but any row ordering, ready-count interpretation, state label, or representative next-work cue should be based on the same dependency-aware view used by mission status.

## Outcome

- `atelier mission list` continues to sort missions by mission-level actionability and status rather than dumping issue rows.
- Any displayed or derived next-work cue for a mission uses blocker-aware issue ordering from the mission's visible work.
- Any displayed work-state cue uses readable action state rather than duplicate `category/status` strings.
- Ready, blocked, evidence-gap, and workflow-state summaries remain compact and consistent with `atelier mission status`.
- Empty or filtered mission-list output keeps the existing next-command guidance.

## Evidence

- CLI test or transcript shows mission list actionability or representative next-work output agrees with mission status for a mission containing a visible blocker chain.
- Regression test or transcript proves mission list remains compact and does not become a full issue listing.
- CLI transcript or focused test for `atelier mission list` proves human rows do not contain duplicate category/status tokens.
- CLI test or transcript shows filtered or empty mission-list guidance still points to the correct follow-up commands.
- `atelier lint atelier-kzfl`, `atelier export --check`, focused cargo tests, and `git diff --check` pass.

## Notes

If implementation discovery shows `mission list` has no issue-level ordering surface left to change, close this with proof that it already derives all issue-level cues from the shared mission summary path.
