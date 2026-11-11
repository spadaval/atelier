---
schema: atelier.item
schema_version: 3
id: atelier-tvfd
title: 'Show write commands'' full forms in atelier --help: agents ran 541 help lookups to find --role, --kind and section names'
status: ready
mission: atelier-icv4
labels:
- cli-output
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T05:11:56.495669950Z
trivial: null
caused_by: null
created_at: 2026-11-09T05:11:56.495669950Z
---

## Goal

Orchestrators, implementers, planners and validators ran 541 `--help` lookups in five days to find the flags of the commands they write records with, about 4 per orchestrator session. Across all roles, 51 lookups repeated one made earlier in the same session. Show the full form of each write command, with its allowed values, in `atelier --help`, the help agents already read first.

references/atelier.md sends agents to "`atelier <command> --help` each command's record sections, flags and rules", and the top-level help's Common commands give nine short forms, of which only `review approve` is a write command with its flags. What agents used right after a lookup shows what they were missing:

- `issue link`: `--role` (32 of 34 next runs).
- `issue note`: `--kind`, with `--reason` and `--rejected` for decisions (39 of 42).
- `issue create`: `--body` with its section names, and `--issue-type`.
- `evidence record`: `--target`, `--kind`, `--result`, `--producer`, and `--path` or `--uri`.
- `review disposition`: `--decision` and `--identity`.

## Done when

- [ ] `atelier --help` shows one line for each write command agents use (issue create, update, link, note and transition; evidence record; review finding, approve, disposition, defer and trivial), each with its required flags and their allowed values, such as `atelier issue link <id> <target> --role advances|blocked_by|related|...`.
- [ ] A test parses each shown line with the CLI's own parser, placeholders filled, so the help fails its test when a flag or value changes.
- [ ] `atelier --help` stays under 80 lines.
- [ ] references/atelier.md points to `atelier --help` for these forms.

## Scope

`crates/atelier-cli/src/main.rs` (top-level help), a test beside it, `skills/agent-factory/references/atelier.md`.

## Context

Evidence: transcripts 2026-11-04 to 2026-11-09; lookups by role: orchestrators 325, implementers 151 (21 of them in one Codex session), planners 46, validators 19. Most common, all roles: `issue note --help` 66, `issue create --help` 56, `evidence record --help` 45, `issue link --help` 44, `review disposition --help` 36. atelier-rh0d fixes `issue link`'s role defaults and errors.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-rh0d (Fix link role defaults and role errors in issue link and unlink)
