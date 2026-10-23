---
created_at: "2026-10-17T01:10:57.978702662+00:00"
id: "atelier-ft88"
issue_type: "bug"
labels:
- "cli-output"
- "land"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep Stop-hook adapter errors apart from landing failures"
updated_at: "2026-10-23T14:14:23.025856246+00:00"
---

## Goal

Malformed hook input is reported as a landing invariant and blocks the agent (crates/atelier-cli/src/commands/session.rs:625-637):
- garbage stdin gives `{"decision":"block","reason":"Atelier landing failed: expected value at line 1 column 1. Fix the named invariant or hand off unfinished work honestly; Atelier never creates commits."}`;
- a missing `cwd` gives `hook payload requires cwd. Fix the named invariant…`;
- a SessionStart payload sent to land gives `expected Stop hook payload. Fix the named invariant…`.

Every real block reads `…then retry.. Fix` with a double period, and ends in static guidance, which ADR 0022 Decision 2 removes from command output. For a bad `cwd`, SessionStart says `Atelier context unavailable: No such file or directory (os error 2). Run atelier check and repair the named records.`, which is wrong advice.

## Done when

- Payload or adapter errors produce a non-blocking `systemMessage`: `Atelier hook misconfigured: <error>; check .claude/settings.json`.
- The block reason is the land error verbatim, with no double period and no static trailer.
- SessionStart advice matches the error class.
- Hook tests cover garbage input, missing `cwd`, the wrong event and a real failure.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. Payload errors from read_hook_payload (session.rs:482-495) block as 'Atelier landing failed: …'; refusal text gets a double period and static trailer (session.rs:636). SessionStart bad cwd advises atelier check (status.rs:32-35). Overlaps atelier-467y; same branch.
