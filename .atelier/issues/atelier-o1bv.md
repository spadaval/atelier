---
created_at: "2026-10-17T09:26:58.654226601+00:00"
id: "atelier-o1bv"
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
title: "Ignore COLUMNS when output is redirected"
updated_at: "2026-10-17T09:26:58.654226601+00:00"
---

## Goal

`stdout_width` (`crates/atelier-cli/src/human_output.rs`, about line 62) takes `COLUMNS` even when stdout is not a terminal, so `COLUMNS=30 atelier work ready | cat` wraps every row. That contradicts `docs/product/human-cli-output.md` ("Redirected output has no width limit"), which agents rely on to read one line per item. Found in review of atelier-teld (finding atelier-je55); already true before it.

## Done when

- Redirected output has no width limit whatever `COLUMNS` says; a terminal still uses it.
- A test runs a view with `COLUMNS` set and stdout redirected and finds no wrapped rows.
