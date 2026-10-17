---
created_at: "2026-10-17T01:10:57.967698603+00:00"
id: "atelier-6gpl"
issue_type: "feature"
labels:
- "cli-output"
- "issue-commands"
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
status: "backlog"
title: "Render issue transition options one line per option"
updated_at: "2026-10-17T01:10:57.967698603+00:00"
---

## Goal

`atelier issue transition <id>` (`render_issue_transition_options`, `crates/atelier-cli/src/commands/workflow.rs:316-370`) uses 6-8 lines per option, each with its own `Commands\n--------` heading. It prints `From: backlog, todo, blocked, in_progress` when the current status is already known. A todo task needs 30 lines for 4 options. Problems with the labels:
- A transition that only needs `--reason` (close, supersede) is labelled `[blocked]` under "Failed Requirements: required field close_reason", which reads as a gate failure.
- `--verbose` prints `Decision: blocked` under the Failed Requirements heading. On a decision record that collides with the `decision` type and its `decide` transition.

Errors are just as unhelpful: `transition 43m1 close` from todo prints `Transition 'close' is not available from status 'todo' for issue atelier-43m1` (`workflow.rs:150-163`) without the route. Agents pipe the options view to head, sed or grep (about 25 of 189 runs in astromech, 15 of 95 in atelier), including `sed -n '/close \[/,/Commands/p'` to cut out one option. Astromech agents chain `transition X start -q >/dev/null; transition X close --reason …` because they have to work the route out themselves.

## Done when

- Default output is a heading line `atelier-db6h todo — transitions:` and then one line per option, e.g. `✓ start → in_progress  atelier start db6h`, `· close → done  needs --reason "…"`, `⚠ start  blocked by atelier-bw0b [todo] "Record ADR 0022…"`.
- Options that only lack an argument are not marked blocked.
- `--verbose` keeps validators and descriptions, without a `Decision:` label.
- An unavailable transition error lists the options available now and the shortest route, e.g. `close needs in_progress or review; from todo: start, then close`.
- Rendering and CLI tests are updated.

## Context

atelier-7u4o compacts the readiness section inside `issue show`; this is the standalone view. ADR 0022 one line per item.
