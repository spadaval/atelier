---
created_at: "2026-10-17T09:36:28.855906025+00:00"
id: "atelier-4775"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-4775
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
closed_at: "2026-10-17T09:48:48.598318868+00:00"
status: "done"
title: "Agree on close readiness in brief and full issue show"
updated_at: "2026-10-17T09:48:48.598318868+00:00"
---

## Goal

Mission validation (evidence atelier-pnha) found that on an in-progress issue, brief `issue show` lists `close --reason` as allowed, while `issue show --full` Transition Readiness says `close: blocked - missing required field close_reason`. The contract treats a transition whose only gap is a required argument as allowed.

## Done when

- Both views report a transition that needs only an argument the same way, as allowed with its flag.
- A test pins both views for that case.

## Close Reason

Brief issue show, issue show --full Transition Readiness and issue transition <id> now all report a transition whose only gap is a required argument as allowed with its flag. test_argument_only_transition_is_allowed_in_every_issue_view pins all three views and the quiet token. Evidence: atelier-6398 (suite), atelier-upnf (fmt), atelier-vrnf (diff check).
