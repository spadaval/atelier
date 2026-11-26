---
schema: atelier.item
schema_version: 3
id: atelier-40e4
title: Run Sol in Pi with atelier's role, guard and Done-when reminders, to compare against Codex
status: in_review
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

- [x] A Pi thread in an atelier checkout is told who it is at the start of a session: `atelier status`, plus the goal and unticked Done when lines of its item, or the outcome and success criteria of the mission it leads.
- [x] A handoff the guard denies is blocked with the guard's reason, including one sent from inside a `codemode` script; an allowed handoff goes through.
- [x] A reviewer's `edit` and `write` calls are blocked, an implementer's are not, and the role follows `atelier review` run mid-session.
- [x] The records are sent again every 25 turns and after a compaction, at the tail of the conversation, with Pi's own summary kept.
- [x] A long-running command can be started, read and stopped while the agent keeps working.
- [x] The extension loads in Pi 1.6.1 and a real Sol turn in an atelier checkout shows the start message.

## Scope

In: `pi-extension/`. Out: any change to the `atelier` CLI; moving `handoff-guard` into atelier (deferred until the comparison has recalibrated it); thread caps and custom compaction summaries, which the council review cut from the first version; a CI job for the extension, which is worth adding only if the comparison keeps it.

## Report

A Pi thread now starts knowing its item, has its handoffs checked by the same guard as Codex, and is reminded of what is left every 25 turns and after compaction.

What was verified, and how:

- `node --test pi-extension/test.mjs` passes 12 tests against a stand-in for Pi: the start message, the guard's allow and deny, the reviewer block and its refresh after `atelier review`, the reminder cadence and the post-compaction reminder, and a real `tmux` job started, read and stopped.
- Pi 1.6.1 with `gpt-6.1-sol`, run in this item's checkout, answered from the start message alone with the item's id and its first unticked Done when line. A second turn called `background_start`, `background_read`, `background_stop` and `codemode` and each returned as expected.
- In T3 Code, a Pi child was asked to send a run-together handoff through `mcp__t3-code__delegate_task`, once directly and once from a `codemode` script. Both were blocked with the guard's reason, no task started, and both attempts are in `~/.local/state/handoff-guard/attempts.jsonl` tagged `pi`.

What was not verified: the reminder and the post-compaction message in a real long session (only the stand-in fired those events), and the reviewer block with a real reviewer thread. The comparison run will exercise the first.

What a reader should know:

- It changes no Rust and adds no `atelier` subcommand. It reads `atelier status`, `atelier show` and the record files. Two things are read from prose: the role line (`You are the reviewer of ...`) and the `led by <thread>` line. A wording change in either silently turns off the reviewer block or the lead's reminder; `test.mjs` holds the current wording.
- The guard fails open, as Codex's hook does, so the two arms of the comparison behave alike. A guard that did not answer is written to the session as an `atelier-guard-error` entry.
- No CI job runs `test.mjs`. That is deliberate while the extension is an experiment, and is the first thing to add if the comparison keeps it.
- The council review of the plan (2026-11-26) cut thread caps and replacement of Pi's compaction summary from this version, and moved role limits from `setActiveTools` to `tool_call`, because T3's own Pi extension manages the active tool list.

### Responses

- 1-tmm3.1: fixed in 3e47fac73
- 1-tmm3.2: fixed in 3e47fac73
- 1-tmm3.3: fixed in 3e47fac73
