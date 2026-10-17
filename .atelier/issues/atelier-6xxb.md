---
created_at: "2026-10-17T01:10:57.990209220+00:00"
id: "atelier-6xxb"
issue_type: "bug"
labels:
- "cli-output"
- "cross-cutting"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-6xxb
priority: "P2"
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
closed_at: "2026-10-17T07:33:12.549318008+00:00"
status: "done"
title: "Name the real ID in every next-step hint and error"
updated_at: "2026-10-17T07:33:12.549318008+00:00"
---

## Goal

Hints print placeholders even when the ID is in scope, so the reader has to run another command to fill them in:
- `status.rs:139` "Inspect selected work transitions: atelier issue transition `<issue-id>`" (the top ready ID is known)
- `session.rs:188` prime: "Select ready work with atelier work ready, then atelier start `<id>`"
- `work.rs:95` "Open mission dashboard: atelier work mission `<mission-id>`" (mission IDs are known)
- `human_output.rs:448` "Inspect a record: atelier issue show `<issue-id>`", printed even under "No issues match the selected filters."
- `issue.rs:527` "Execute a transition: atelier issue transition atelier-x `<transition>`" (allowed transitions are listed just above)
- `issue.rs:932` "Nothing to update ... `atelier issue note <id>`"
- `relate.rs:67`, `workflow_validation.rs:130`, `evidence.rs:535`: `--target issue/<id>`
- `workflow_policy.rs:53/67/95`: `issue atelier-xmtr cannot be child of mission atelier-1w8c; link mission work with atelier issue link <issue-id> <mission-id> --role advances` (both IDs are already in the sentence)
- `merge.rs:59` and `session.rs:450`: "run atelier start and commit its metadata" with no ID
- `evidence.rs:597`: `atelier evidence show <evidence-id>`

Hints are also quoted three ways: in backticks, bare, or in single quotes ('atelier init').

## Done when

- Each listed hint renders the concrete ID: the top ready item, actual mission IDs, the issue's allowed transition names, the issue and mission named in the error, or the merge owner. For example `atelier issue link atelier-xmtr atelier-1w8c --role advances`.
- Hints with no rows to point at are omitted.
- One quoting style (backticks) is used.
- A test renders status, prime, issue list (empty and non-empty), issue show and the hierarchy error from fixtures, and asserts no `<...-id>`/`<transition>` placeholder appears where an ID exists. Placeholders stay only where the ID truly isn't known (for example unowned-branch errors in session.rs:258/294/512/529).

## Close Reason

Hints, footers and error recovery steps name the real ID where the command knows it (top ready item, top mission, first list row, runnable transitions, hierarchy/link endpoints, merge/land owner, evidence target); empty-row footers are omitted; in-sentence commands use backticks; rule added to docs/product/human-cli-output.md; pinned by test_hints_name_real_ids_instead_of_placeholders.
