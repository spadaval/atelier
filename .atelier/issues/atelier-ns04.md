---
created_at: "2026-10-17T01:10:57.979960270+00:00"
id: "atelier-ns04"
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
status: "todo"
title: "Point land <id> at the owner's branch, and name unknown IDs"
updated_at: "2026-10-23T15:47:17.794696773+00:00"
---

## Goal

- On master, `atelier land 7b6e` for an item started on `task/atelier-7b6e` prints `land_work_untraceable: owner has no committed branch metadata; run atelier start and commit the record`. That is wrong: the work is started, its branch metadata is committed on that branch, and status lists the branch.
- `atelier land 1w8c` (a mission) prints the same "run atelier start" advice, but missions are not started.
- `atelier land jb2t` (a child that exists only on its branch) and `atelier land nope` both print `landing issue not found`, which does not name the reference.
- Other untraceable hints say `run atelier start <id>` even when the ID is known. One example is an unowned branch that changes `.atelier/issues/atelier-mp6v.md`.

## Done when

- When a local branch owns the ID, land says `atelier-7b6e is owned on task/atelier-7b6e; git switch task/atelier-7b6e && atelier land`.
- A mission gets `missions have no branch; land one of their items`.
- Not-found says `no issue matches "nope"`.
- Hints name the ID, for example `atelier start atelier-mp6v`.
- Tests cover each case.

## Context

`owner()` is at crates/atelier-cli/src/commands/session.rs:11-36 and the untraceable messages at 450 and 528-530. Owner branches are found with `git::branch_lines_for` (crates/atelier-cli/src/commands/git.rs:465).

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: partly done: unknown IDs are named. Still: land X on the base and land <mission> both say 'has no committed branch metadata; run atelier start' (session.rs:341); unowned hints say 'atelier start <id>' (:104,140,403,406,420). Use git::branch_lines_for; mirror merge_mission (merge.rs:415).
- 2026-10-23 [decision] land <mission> refuses with a new land_mission code and 'Next: atelier work mission <id>', mirroring merge_mission; hints with no derivable ID say 'atelier start and its item's ID' in words rather than a <id> placeholder, and the unowned-branch hint names the first changed issue record's ID.
  Reason: A land gate may own codes; a fix line must be runnable with real IDs, so where no ID is knowable the hint is prose.
  Rejected: Reusing land_work_untraceable for missions
