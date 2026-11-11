---
schema: atelier.item
schema_version: 3
id: atelier-o1bv
title: Ignore COLUMNS when output is redirected
status: dropped
mission: atelier-4pzj
labels:
- cli-output
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T09:26:58.654226601Z
---

## Goal

`stdout_width` (`crates/atelier-cli/src/human_output.rs`, about line 62) takes `COLUMNS` even when stdout is not a terminal, so `COLUMNS=30 atelier work ready | cat` wraps every row. That contradicts `docs/product/human-cli-output.md` ("Redirected output has no width limit"), which agents rely on to read one line per item. Found in review of atelier-teld (finding atelier-je55); already true before it.

## Done when

- [ ] Redirected output has no width limit whatever `COLUMNS` says; a terminal still uses it.
- [ ] A test runs a view with `COLUMNS` set and stdout redirected and finds no wrapped rows.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: atelier-je55

## Report

Dropped: The 0.5 rewrite removed the width handling this describes: no code under crates/ reads COLUMNS, and output is never wrapped.
