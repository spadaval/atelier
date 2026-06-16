---
created_at: "2026-06-16T15:59:33.803325324+00:00"
id: "atelier-nqjc"
issue_type: "feature"
labels:
- "cli"
- "dependencies"
- "issue"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-em15"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Sort issue show subissues by blocker order"
updated_at: "2026-06-16T15:59:33.803325324+00:00"
---

## Description

Update issue detail output so the subissue section of a parent issue or epic guides agents through prerequisites before dependents and uses readable row state. Parent detail is often the first place an orchestrator checks whether an epic can advance.

## Outcome

- `atelier issue show <parent-or-epic>` lists child issues in blocker-aware order when open blocker edges exist between visible children.
- Subissue rows use readable action state rather than duplicate `category/status` tokens.
- Existing status, priority, evidence, and next-command cues remain visible where they add information.
- Exact blocker IDs remain available in blocker/detail sections or drill-down commands, rather than being the primary subissue row signal.
- Hidden blockers outside the child set remain represented by blocker cues and do not create extra child rows.
- Closed child blockers no longer force completed prerequisite work above current actionable children unless the existing grouping intentionally shows closed work separately.

## Evidence

- CLI test or transcript shows an epic with contract, implementation, and validation children where `issue show` prints the blocker before the dependent child.
- CLI transcript or focused test for `atelier issue show <parent>` proves subissue rows use readable state and do not contain duplicate category/status tokens.
- CLI test or transcript shows a child blocked by an external issue keeps the external blocker cue without inserting the external issue as a child.
- Focused regression proves existing subissue metadata and next-command cues remain present.
- `atelier lint atelier-nqjc`, `atelier export --check`, focused cargo tests, and `git diff --check` pass.

## Notes

This surface should make epic execution order visible without turning `issue show` into `mission status`.
