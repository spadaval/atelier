---
created_at: "2026-10-17T09:36:28.636174653+00:00"
id: "atelier-u3vj"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-u3vj
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T09:42:38.160751086+00:00"
status: "done"
title: "Make status follow the count and footer rules"
updated_at: "2026-10-17T09:42:38.160751086+00:00"
---

## Goal

Mission validation (evidence atelier-pnha) found `status` breaking the contract in `docs/product/human-cli-output.md`:
- It writes counts in parentheses, `Unassigned Work (7)` and `no label (4)` (`crates/atelier-cli/src/commands/status.rs`, about line 568), where `work missions` writes `Unassigned: 1 open issue`.
- Its "Other Local Work Branches" section prints after `Next`, so the footer is not last.
- Mission scope counts are labelled `Scope:` in `work missions` and the Mission Rollup but `Work:` in `work mission`, `work parent` and brief `issue show`.

## Done when

- Counts in headings follow the Counts rule, the `Next` section is last, and one label names scope counts everywhere.
- Rendering tests cover each.

## Close Reason

status counts unassigned work in its summary (Unassigned work: N) with no count in the heading; label groups in status and work missions read '<label>: N open issues'; Other Local Work Branches precedes the Next footer, which is last; work mission, work parent and brief issue show label scope counts Scope:. Rendering tests cover each; evidence atelier-mtt5, atelier-mwtj, atelier-y9kg.
