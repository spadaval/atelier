---
created_at: "2026-10-17T08:32:32.044650208+00:00"
id: "atelier-y0en"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Report an invalid workflow policy once"
updated_at: "2026-10-17T08:32:32.044650208+00:00"
---

## Goal

With an invalid committed `.atelier/workflow.yaml`, `land` and `merge --check` print the parse failure three times: once in their error, and twice in older warnings from `crates/atelier-app/src/base_policy.rs` ("Invalid working policy:" and "Invalid policy at <commit>:"). The repeats bury the real error. Found in review of atelier-91w1 (finding atelier-9rhe).

## Done when

- An invalid policy is reported once, with one code (`workflow_config_parse_error`) and one `Next:` step.
- A test pins the invalid-policy output of `land` and `merge --check`.
