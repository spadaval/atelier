---
created_at: "2026-11-09T00:45:58.909953386+00:00"
id: "atelier-7yy9"
issue_type: "feature"
labels:
- "fleet-dashboard"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-tu8c"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Record which thread, harness, model and role ran each atelier command"
updated_at: "2026-11-09T00:45:58.909953386+00:00"
---

## Goal

Record which agent did what: every `atelier` command run inside a T3 thread writes one row to a host-wide session database, so the site and metrics can say which thread, harness, provider, model, effort and role worked on each item. Codex models now share the work with Claude, and today nothing records this.

## Done when

- [ ] Commands that act on an item (`start`, `review request`, review findings, approvals and dispositions, `issue note`, `issue transition`, `merge`) write a row to `~/.local/share/atelier/sessions.db` (SQLite, WAL) with: time, repository (Git common directory), item, command, role derived from the command (implementing, reviewing, planning, orchestrating), commit, T3 thread ID, provider session ID, harness, and model and effort where the environment gives them. Tests run commands under each harness's environment and check the row.
- [ ] Harness detection follows the 2026-11-08 spike: Claude Code (`CLAUDE_CODE_ENTRYPOINT`, `AI_AGENT`, session `CLAUDE_CODE_SESSION_ID`, effort `CLAUDE_EFFORT`); Codex (`CODEX_VERSION`, session `CODEX_SESSION_ID`); Grok (`GROK_AGENT`, session `GROK_SESSION_ID`). `T3CODE_THREAD_ID` is recorded when set; Claude and Codex sessions get it, Grok sessions do not, so a Grok row carries only its session ID and the site resolves the thread through T3.
- [ ] Outside a T3 thread or harness the command writes no row, and a missing, locked or unwritable database never makes a command fail or slow down noticeably: it prints at most one warning. Tests cover a read-only database path and a held lock.
- [ ] `merge`, `land`, `check` and `doctor` never open the database (a test runs them with the database path pointing at an unreadable file and checks they behave as without it).
- [ ] The database location and schema are documented in `docs/architecture/`.

## Scope

Recording only. No site pages, no T3 lookups, no token usage.

## Context

ADR 0030 governs. No harness puts the model name in the environment, so the site fills it from T3's thread records. T3 links each provider's own session ID to its thread (checked in T3's provider-thread records on 2026-11-08 for Claude, Codex and Grok).
