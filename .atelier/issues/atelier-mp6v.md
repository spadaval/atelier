---
created_at: "2026-10-16T23:52:05.622714060+00:00"
id: "atelier-mp6v"
issue_type: "feature"
labels:
- "orientation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
  - kind: "review"
    id: "atelier-4cid"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T17:33:00.279904244+00:00"
status: "done"
title: "Stop the Stop hook pushing commits on master for uncommitted records alone"
updated_at: "2026-10-23T17:33:00.279904244+00:00"
---

## Goal

`atelier land --hook claude` runs at the end of every Claude Code turn. On the base branch with one uncommitted `.atelier/` record and no other problem it returns `decision: block` ("commit canonical records before continuing"), which pushes the agent to commit on master without being asked. Found while installing the hooks in this repository (review finding atelier-4cid on atelier-bw0b).

## Done when

On the base branch, when the only failed invariant is uncommitted canonical records, the Stop hook returns a warning (exit 0, no block) naming the paths, while plain `atelier land` still fails. Every other failure still blocks. A hook test covers both cases, and `docs/product/sessions-and-hooks.md` describes the rule.

## Context

`crates/atelier-cli/src/commands/session.rs` (land hook adapter). Hook cost per turn (about 5.8 s here) is being addressed by atelier-37uq.

## Unknowns

Whether the same relaxation should apply on work branches, where uncommitted records usually belong with the work in progress.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still accurate, but the quoted message is outdated (now 'land_records_uncommitted: … differs from pinned HEAD'). read_record_files aborts land_checked (session.rs ~205) before other checks, so it needs land to report uncommitted records as one failure among all (atelier-0uaj) first. Decided on atelier-rc8j: the relaxation applies on the base only.
- 2026-10-23 [decision] The hook relaxes only a refusal land returned after its gates ran on the base branch with no owner (GatesRefused.ran_on = Base) whose every failed check code is land_records_uncommitted; a refusal that stopped before the gates always blocks.
  Reason: Decides from land's structured failed-check list and where land ran, not from text; a stopped refusal has another code by construction, so it cannot be records-only.

## Close Reason

On the base branch, when land's only failed check is `land_records_uncommitted`, the Stop hook returns a systemMessage (exit 0, no block) that says so and carries land's refusal naming the records and their commit; plain land still refuses, any other failure on the base blocks, and uncommitted records on a work branch block.

### Deviations
None.

### Decisions
See the decision note: the decision reads land's structured failed-check list (`GateReport.checks`) and where land ran (`GatesRefused.ran_on`), set only after the gates ran.

### Surprises
A done claim made directly on a base with no upstream is not a land failure there, so the mixed-failure case uses a committed policy that does not parse.

### Open items
None.
