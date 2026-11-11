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
title: "Give SessionStart one line of correct advice outside an Atelier repository"
updated_at: "2026-11-07T21:03:23.755608551+00:00"
---

## Goal

A SessionStart hook whose cwd is not inside an Atelier repository prints `Atelier context unavailable: Not an Atelier repository (or any parent). Run `atelier init` first.. Run `atelier check` and repair the named records.`: a double period, and advice to repair records that do not exist (checked 2026-11-07 with `atelier status --hook claude` and a payload whose cwd is a plain directory). Say once that the directory is not in an Atelier repository, so there is no context to show, as the Stop hook now does.

## Done when

- [ ] A hook test: SessionStart with a cwd outside any Atelier repository gives one line naming the directory and saying there is no Atelier context there, with no double period and no `atelier check` advice, and exits 0.

## Context

atelier-md0v took the Stop half: `atelier land --hook claude` outside a repository now says there is nothing to land and exits 0. This item keeps the SessionStart half (decision atelier-mzpn). crates/atelier-cli/src/commands/status.rs (`--hook`). Found by acceptance validation atelier-1o56 (S5 scope gap; atelier-ft88 covered only a cwd that cannot be entered).
