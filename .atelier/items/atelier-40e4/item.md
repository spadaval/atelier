---
schema: atelier.item
schema_version: 3
id: atelier-40e4
title: Run Sol in Pi with atelier's role, guard and Done-when reminders, to compare against Codex
status: building
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- ca01f4b0-4d65-48db-9ae6-ba19251fd343
ready: null
trivial: null
caused_by: null
created_at: 2026-11-26T22:44:53.420011350Z
---

## Goal

Sol's handoffs degrade and it loses the plot on long threads, and under the Codex CLI we can neither read its compaction nor change it. A Pi extension lets a Sol thread run under atelier's rules in a harness we control, so the same mission can be run once through Codex and once through Pi and compared. This item is the extension; the comparison is run separately.

## Done when

- [ ] A Pi thread in an atelier checkout is told who it is at the start of a session: `atelier status`, plus the goal and unticked Done when lines of its item, or the outcome and success criteria of the mission it leads.
- [ ] A handoff the guard denies is blocked with the guard's reason, including one sent from inside a `codemode` script; an allowed handoff goes through.
- [ ] A reviewer's `edit` and `write` calls are blocked, an implementer's are not, and the role follows `atelier review` run mid-session.
- [ ] The records are sent again every 25 turns and after a compaction, at the tail of the conversation, with Pi's own summary kept.
- [ ] A long-running command can be started, read and stopped while the agent keeps working.
- [ ] The extension loads in Pi 1.6.1 and a real Sol turn in an atelier checkout shows the start message.

## Scope

In: `pi-extension/`. Out: any change to the `atelier` CLI; moving `handoff-guard` into atelier (deferred until the comparison has recalibrated it); thread caps and custom compaction summaries, which the council review cut from the first version; a CI job for the extension, which is worth adding only if the comparison keeps it.
