---
created_at: "2026-10-17T01:10:57.964569101+00:00"
id: "atelier-pu2i"
issue_type: "feature"
labels:
- "cli-output"
- "issue-commands"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show open work before done work in issue list"
updated_at: "2026-10-17T01:10:57.964569101+00:00"
---

## Goal

`atelier list` with no flags shows only open work and leaves done and dropped items out, but says nothing of what it left out: the list ends after its last open row, with no count of the hidden items and no command to see them (review finding atelier-p4v4, reproduced on the new binary in a scratch clone, where `list --done` showed the one done item `list` hid). Under the old tool, agents worked around a list that hid open records with `--limit 200 | grep -v ' done '` (4 runs) and `--status todo --limit 100 | grep` (9+ runs).

## Done when

- [ ] `list` with no flags ends with a line counting the done and dropped items it leaves out and naming the command that shows them, e.g. `67 done or dropped not shown: atelier list --done`.
- [ ] A CLI test with done items shows the line, and one with none shows no such line.

## Scope

The last line of `list` and its test.

## Context

ADR 0022 (dense Markdown; piping means a view is missing). Related: atelier-fpxc (text search), atelier-deyy (dense rows).

Rewritten for the new tool in the cutover sort (atelier-5zre): `issue list` became `list`, which already lists open work only and has no title or padding block, so those parts left; the truncation line went, since `list` does not cut rows; the footer counting hidden records remains, naming `list --done`. The inventory doc is replaced by the design record.
