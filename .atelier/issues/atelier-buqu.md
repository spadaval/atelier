---
created_at: "2026-10-23T14:13:38.067987247+00:00"
id: "atelier-buqu"
issue_type: "feature"
labels:
- "cli-output"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-467y"
  - kind: "issue"
    id: "atelier-ft88"
  - kind: "issue"
    id: "atelier-i80r"
  - kind: "issue"
    id: "atelier-mp6v"
  - kind: "issue"
    id: "atelier-u4ql"
  - kind: "issue"
    id: "atelier-zegg"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-0uaj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Stop hook blocks only on failures the agent must act on now"
updated_at: "2026-10-23T17:15:36.778845992+00:00"
---

## Goal

The Stop hook blocks a turn only for landing failures the agent must act on now: adapter and payload errors are reported apart and do not block; on the base, uncommitted records alone do not block; a review still pending on the branch's own item is reported without blocking; a record that needs a newer binary says to reinstall Atelier; a refusal carries one code and one recovery step.

## Done when

Hook tests for each case: bad payload, records-only on the base, pending review, mixed pending review plus another failure (blocks), newer schema_version (reinstall message, in `status` and the hook), and a refusal's single recovery step. Delivers S5. Author proof plus independent adversarial review.

## Scope

Children: atelier-ft88, atelier-467y, atelier-u4ql, atelier-mp6v, atelier-i80r. `land` itself still refuses in every case; only the hook's blocking decision changes. No new record types.

## Context

Hook path `crates/atelier-cli/src/commands/session.rs:482-495, 625-641`; schema checks `crates/atelier-records/src/lib.rs:811-817`, `review.rs:79-83`; `docs/product/sessions-and-hooks.md`. Uses the per-check failure list the earlier rc8j land increments produce. Decisions on atelier-rc8j's 2026-10-23 notes define "pending review" and limit the records-only relaxation to the base.

## Notes

- 2026-10-23 [note] 2026-10-23 plan: since this increment was written, atelier-h1nc gave the Stop-hook reason the land lines and verdict on their own lines (the trailer no longer glues onto a fix; part of atelier-467y), and atelier-0uaj made land report preconditions (uncommitted records, dirty completion, operation in progress) as failed checks alongside the gates, so mp6v and i80r can classify the full failure list rather than stopping at the first. atelier-zegg (name cherry-pick/revert in progress; warn when append-only recovery drops a merge) joins after atelier-0uaj merges.
