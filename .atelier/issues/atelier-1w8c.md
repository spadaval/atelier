---
created_at: "2026-10-17T00:31:28.515786218+00:00"
id: "atelier-1w8c"
issue_type: "mission"
labels:
- "orientation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-7n5u"
    type: "advances"
  - kind: "issue"
    id: "atelier-7u4o"
    type: "advances"
  - kind: "issue"
    id: "atelier-deyy"
    type: "advances"
  - kind: "issue"
    id: "atelier-ihns"
    type: "advances"
  - kind: "issue"
    id: "atelier-jjvc"
    type: "advances"
  - kind: "issue"
    id: "atelier-majk"
    type: "advances"
  - kind: "issue"
    id: "atelier-sdb0"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Orient a session from one view: ADR 0022 follow-ups"
updated_at: "2026-10-17T02:21:36.549562785+00:00"
---

## Outcome

An agent starting a session runs `atelier status` and nothing else to learn where it is, what is in flight on other branches, what is ready, and what the open work is about between missions. Every Atelier view reads as dense Markdown per ADR 0022: one line per item, brief detail by default, and no routine piping through head, grep, or sed. This mission runs with decision capture and implementer reports switched on as soon as atelier-r6bt's S1 and S2 land, so it is the real mission atelier-r6bt's debrief (S7) is judged against.

## Success criteria

- S1: `atelier status --hook claude` is the SessionStart hook and shows what `prime` showed for an owner plus the top ready items; `prime` is a deprecated alias (atelier-sdb0). Shown by the moved session tests.
- S2: `issue show` is brief by default, takes several IDs, and has `--full` (atelier-7u4o). Shown by CLI tests.
- S3: `work` views and `status` use one dense line per item with ✓ and ⚠ marks, and `human-cli-output.md` states the rule (atelier-deyy). Shown by rendering tests.
- S4: With no open mission, `status` and `work missions` name the last closed mission and group unassigned work by label (atelier-ihns). Shown by CLI tests.
- S5: `work ready` marks items already in flight on another local branch (atelier-7n5u). Shown by a CLI test.
- S6: Claude Code transcripts from sessions after S1-S5 land show atelier commands piped to head, grep, or sed in fewer than one run in five, against about four in five before (ADR 0022 context). Shown by a count over the transcripts, recorded as evidence.

## Background

ADR 0022 (atelier-bw0b, on branch task/atelier-bw0b, proposed) is the governing decision and must be accepted before this mission leaves draft. atelier-bw0b itself is not linked here because its record is being edited on its branch. Deferred from this cycle: text search (atelier-fpxc) and unblock ranking (atelier-fdxj), which change no view a human or agent reads before, during, or after a mission. Recent-activity at session start (atelier-majk) waits on the shared record-history query atelier-r6bt builds, and is linked here in backlog.

## Notes

- 2026-10-17 [note] 2026-10-16: atelier-o3lm was split by surface; the status-page and work-view items it held now sit in atelier-4pzj, which follows this mission.
- 2026-10-17 [note] 2026-10-16: Activated. ADR 0022 was accepted and atelier-bw0b merged to master, so the Background's 'proposed, on branch task/atelier-bw0b' no longer holds. Plan: sdb0 and 7u4o in parallel (disjoint surfaces), then 7n5u, ihns, deyy in sequence; jjvc stays in backlog until adopters' hooks stop calling prime.
