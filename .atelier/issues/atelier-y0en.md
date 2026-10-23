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
updated_at: "2026-10-23T15:47:09.435870121+00:00"
---

## Goal

With an invalid committed `.atelier/workflow.yaml`, `land` and `merge --check` print the parse failure three times: once in their error, and twice in older warnings from `crates/atelier-app/src/base_policy.rs` ("Invalid working policy:" and "Invalid policy at <commit>:"). The repeats bury the real error. Found in review of atelier-91w1 (finding atelier-9rhe).

## Done when

- An invalid policy is reported once, with one code (`workflow_config_parse_error`) and one `Next:` step.
- A test pins the invalid-policy output of `land` and `merge --check`.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed: land and merge --check print the policy parse error 3 times (base_policy.rs:47,74 printed at session.rs:263 and merge.rs:458, then Caused by from session.rs:210/merge.rs:493); land's verdict is bare 'land: refused'.
- 2026-10-23 [decision] An invalid policy's fix is prose (fix .atelier/workflow.yaml and commit it); the fix helper skips a Next: with no atelier/git command.
  Reason: Only a person can edit the policy; a restore of the last valid version could silently revert intended changes.
  Rejected: git restore --source <last valid> -- .atelier/workflow.yaml
- 2026-10-23 [decision] main.rs no longer prints the working-policy fallback warnings for land, doctor and merge (merge --sync still does); land keeps a base policy's fallback notes and drops its own candidate's.
  Reason: The gates report the policy they read with their checks, so the same parse error was printed three times.
