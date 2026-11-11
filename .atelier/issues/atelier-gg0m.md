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

`atelier status` outside an Atelier repository prints `atelier: cannot read this repository's Atelier config: <dir>/.atelier/config.toml: No such file or directory (os error 2)` and exits 2 (checked 2026-11-11 with the new binary in a converted scratch clone). A SessionStart hook that runs there shows an error where a plain note that there is no Atelier context would do.

## Done when

- [ ] A hook test: SessionStart with a cwd outside any Atelier repository gives one line naming the directory and saying there is no Atelier context there, and exits 0.

## Context

atelier-md0v took the Stop half: `atelier land --hook claude` outside a repository now says there is nothing to land and exits 0. This item keeps the SessionStart half (decision atelier-mzpn). crates/atelier-cli/src/commands/status.rs (`--hook`). Found by acceptance validation atelier-1o56 (S5 scope gap; atelier-ft88 covered only a cwd that cannot be entered).

Rewritten for the new tool in the cutover sort (atelier-5zre): the old `status --hook claude` became the new hook's plain `atelier status`; the double period and the `atelier check` advice are gone, the exit status remains.
