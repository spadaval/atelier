---
created_at: "2026-10-17T01:10:57.992665485+00:00"
id: "atelier-ksx6"
issue_type: "bug"
labels:
- "cli-output"
- "cross-cutting"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-ksx6
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
  - kind: "review"
    id: "atelier-tojy"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T08:18:55.137197560+00:00"
status: "done"
title: "Make detail-view footers and readiness depend on record state"
updated_at: "2026-10-17T08:18:55.137197560+00:00"
---

## Goal

`human-cli-output.md` says "Next actions come from actual state ... Avoid generic command menus in every row". Footers are static instead:
- `issue show atelier-37uq` (done) prints `Transition Readiness: options: blocked - Issue atelier-37uq has no configured transitions from status 'done'`. Its Next Commands offers "Edit issue Markdown" (done records can't be revised) and "Execute a transition: atelier issue transition atelier-37uq <transition>".
- After `issue transition <id> close --reason ...`, Next Commands suggests `atelier issue transition <id>`, which then fails with "no configured transitions from status 'done'".
- `work mission atelier-r6bt --ready` offers "List mission ready work: atelier work mission atelier-r6bt --ready", which is the command just run. `work mission 1w8c` offers `--ready`/`--blocked` filters for tables already on screen and never offers `atelier start` on a ready item.
- Done mission `atelier-kgcr` offers "Inspect transitions".
- `issue create` offers "Inspect tracked work transitions" rather than `atelier start <id>`.
- "... 4 more done work items omitted" (`work mission kgcr`, `work parent 1ioo`) names no flag, and `work parent` has none.

## Done when

- On done records, issue show's readiness section reads `Closed <date>: <Close Reason first line>`, and the footer has no edit or transition actions.
- After a close transition, the footer suggests `land` / `merge <owner> --check` rather than `issue transition`.
- No footer repeats the current command or a filter for rows already shown.
- Mission and parent views offer `atelier start <top ready id>`. `issue create` offers `atelier start <new id>`.
- Every "omitted" line names the flag that shows the rest (adding `work parent --all` if needed).
- Rendering tests cover a done issue, a close transition, `work mission --ready`, a done mission, and issue create.

## Context

issue.rs:321 (readiness), issue.rs:510-530 (footer), work.rs:563-585 (`objective_footer`), human_output.rs:373 (omitted line). Relates to atelier-7u4o (brief issue show).

## Close Reason

Footers and readiness now follow record state: done records show 'Closed <date>: <reason>' and offer no edit or transition steps; a close offers land and merge --check for the owner; mission and parent views offer atelier start for the top ready item and drop filter steps; issue create offers atelier start; omitted lines name the command that shows the rest, and work parent gains --all.
