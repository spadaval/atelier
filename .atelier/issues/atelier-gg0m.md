---
created_at: "2026-10-23T18:02:21.962449300+00:00"
id: "atelier-gg0m"
issue_type: "bug"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1o56"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Treat a hook cwd outside an Atelier repository as a hook misconfiguration"
updated_at: "2026-10-23T18:02:21.962449300+00:00"
---

## Goal

A Stop payload whose cwd exists but is not inside an Atelier repository blocks the turn with `Not an Atelier repository … Run `atelier init` first.` and `land: refused`. That is an adapter or configuration error, not a landing failure: report it as the non-blocking `Atelier hook misconfigured` message. SessionStart with that cwd prints `Run `atelier init` first.. Run `atelier check` and repair the named records.`, a double period and the wrong advice.

## Done when

Hook tests: Stop with a cwd outside any Atelier repository gives a non-blocking systemMessage; SessionStart with it gives one line of correct advice.

## Context

Found by acceptance validation atelier-1o56 (S5 scope gap; atelier-ft88 covered only a cwd that cannot be entered). crates/atelier-cli/src/commands/session.rs hook path; status.rs --hook.
