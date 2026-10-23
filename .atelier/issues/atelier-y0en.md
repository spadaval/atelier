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
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
  - kind: "review"
    id: "atelier-9rhe"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Report an invalid workflow policy once"
updated_at: "2026-10-23T15:19:35.744219805+00:00"
---

## Goal

With an invalid committed `.atelier/workflow.yaml`, `land` and `merge --check` print the parse failure three times: once in their error, and twice in older warnings from `crates/atelier-app/src/base_policy.rs` ("Invalid working policy:" and "Invalid policy at <commit>:"). The repeats bury the real error. Found in review of atelier-91w1 (finding atelier-9rhe).

## Done when

- An invalid policy is reported once, with one code (`workflow_config_parse_error`) and one `Next:` step.
- A test pins the invalid-policy output of `land` and `merge --check`.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed: land and merge --check print the policy parse error 3 times (base_policy.rs:47,74 printed at session.rs:263 and merge.rs:458, then Caused by from session.rs:210/merge.rs:493); land's verdict is bare 'land: refused'.
