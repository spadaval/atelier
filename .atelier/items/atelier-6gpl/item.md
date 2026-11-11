---
schema: atelier.item
schema_version: 3
id: atelier-6gpl
title: Render issue transition options one line per option
status: draft
mission: atelier-uf97
labels:
- cli-output
- issue-commands
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.967698603Z
---

## Goal

`atelier issue transition <id>` (`render_issue_transition_options`, crates/atelier-cli/src/commands/workflow.rs) uses about five lines per option and prints `From: backlog, todo, in_progress` when the current status is already known; `--verbose` adds a `Decision: allowed` line that collides with the `decision` type and its `decide` transition. An unavailable transition says only `Transition 'plan' is not available from status 'todo' for issue <id>`, without a route. Checked 2026-11-07. Agents piped the options view to head, sed or grep (about 25 of 189 runs in astromech, 15 of 95 in atelier) to cut out one option.

## Done when

- [ ] Default output is a heading line `atelier-db6h todo — transitions:` and then one line per option, e.g. `✓ start → in_progress  atelier start db6h`, `· supersede → superseded  needs --reason "…"`, `⚠ start  blocked by atelier-bw0b [todo] "Record ADR 0022…"`.
- [ ] `--verbose` keeps validators and descriptions, without a `Decision:` label.
- [ ] An unavailable transition's error lists the transitions available now and, where one exists, the route, e.g. `plan needs backlog; from todo: defer, then plan`.
- [ ] Rendering and CLI tests are updated.

## Context

atelier-jnrh removed the work item's `close`, so work items now have at most four transitions and the close routes the original quoted are gone; a transition that only lacks `--reason` is already shown as allowed (decision atelier-mzpn). atelier-7u4o compacts the readiness section inside `issue show`; this is the standalone view. ADR 0022: one line per item.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: atelier-763x
