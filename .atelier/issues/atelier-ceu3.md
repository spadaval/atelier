---
created_at: "2026-10-17T01:10:57.963010672+00:00"
id: "atelier-ceu3"
issue_type: "task"
labels:
- "cli-output"
- "issue-commands"
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
status: "backlog"
title: "Make issue create and update body and type errors readable"
updated_at: "2026-10-17T01:10:57.963010672+00:00"
---

## Goal

Several `issue create`/`update` errors name internal locations or blame the wrong thing:
- Body section errors print `Missing required issue body section 'Done when' for issue (unknown), section Done when, path .atelier/<issue create --body>` and `Unknown issue body section 'Bogus' for issue (unknown), …`.
- `--issue-type epic` (a typo or a retired type) prints `workflow policy at .atelier/workflow.yaml must declare issue_type 'epic' in issue_types and cover it exactly once in workflows.*.applies_to; valid values: …` (`validate_configured_issue_type`, `crates/atelier-cli/src/commands/issue.rs:726-739`), which blames the policy.
- Plain-text bodies are handled differently by the two commands. `create --body "plain"` makes the text the Goal (`issue.rs:704-714`). `update --body "plain"` fails with `Invalid sectioned --body … Content before first recognized issue body section in .atelier/<issue update --body>`.

## Done when

- Body errors read like `--body is missing "## Done when"` or `--body has unknown section "## Bogus"; task sections: Goal, Done when, Scope, Context, Unknowns, Design, Background`.
- An unknown type reads `unknown issue type 'epic'; types: bug, decision, feature, mission, spike, task, validation`.
- `create` and `update` treat a plain-text body the same way, with the behaviour stated in both `--help` texts.
- CLI tests cover each message.

## Context

Placeholder IDs in hints (e.g. `<issue-id> <mission-id>`) are handled by a separate cross-cutting item.
