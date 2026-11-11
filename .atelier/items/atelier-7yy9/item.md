---
schema: atelier.item
schema_version: 3
id: atelier-7yy9
title: Record which thread, harness, model and role ran each atelier command
status: ready
mission: atelier-bnek
labels:
- agent-attribution
- feature
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T01:04:26.120179815Z
trivial: null
caused_by: null
created_at: 2026-11-09T00:45:58.909953386Z
---

## Goal

Record which agent did what: `atelier` commands run inside a T3 thread write rows to a host-wide session database, so the site and metrics can say which thread, harness, provider, model, effort and role worked on each item. Codex models now share the work with Claude, and today nothing records this.

## Done when

- [ ] Every command that names an item, reads or writes, writes one row to `~/.local/share/atelier/sessions.db` (SQLite, WAL) when run in a T3 thread or a recognised harness: `start`, `issue show`, `issue note`, `issue transition`, `issue create`, `brief`, `debrief`, `review request`, review `record`, `finding`, `approve` and `disposition`, and `merge`. Each row holds time, repository (Git common directory), item, command, commit, T3 thread ID, provider session ID, harness, model and effort where known, and role. Tests run each under each harness's environment and check the row.
- [ ] Role is recorded from, in order: an explicit `--as <role>` flag or `ATELIER_ROLE` (implementing, reviewing, planning, orchestrating, gardening); the command (`start` implementing; `review request` and review events reviewing; `brief`, `issue create` and plan notes planning); else none. Tests cover each source.
- [ ] Harness detection follows the 2026-11-08 spike: Claude Code (`CLAUDE_CODE_ENTRYPOINT`, `AI_AGENT`; session `CLAUDE_CODE_SESSION_ID`; effort `CLAUDE_EFFORT`), Codex (`CODEX_VERSION`; session `CODEX_SESSION_ID`), Grok (`GROK_AGENT`; session `GROK_SESSION_ID`). `T3CODE_THREAD_ID` is recorded when set; Grok sessions lack it, so their rows carry only the session ID.
- [ ] Outside a T3 thread or harness no row is written. A missing, locked or unwritable database never makes a command fail and adds under 50 ms; it prints at most one warning. Tests cover a read-only path and a held lock.
- [ ] Guard isolation: the CLI's session-store module exposes only a write function, called after a command's outcome is decided; no guard code (`merge`, `land`, `check`, `doctor`) can read it. A test under `--features test-hooks` counts store calls and shows none happen before `merge`'s gates decide, and `merge`, `land`, `check` and `doctor` give the same results with the database present, absent or unreadable.
- [ ] The database location, schema and role rules are documented in `docs/architecture/`.

## Scope

Recording only. No site pages, no T3 lookups, no token usage.

## Context

ADR 0030 governs: guard decisions consult only Git, and the CLI only writes this database. No harness puts the model name in the environment, so the site fills it from T3's thread records; T3 links each provider's own session ID to its thread (checked for Claude, Codex and Grok on 2026-11-08). Recording read commands that name an item gives token attribution a start for a reviewer that reads before it writes anything (planning review findings atelier-b9wq and atelier-79wo).
