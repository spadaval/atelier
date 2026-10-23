---
created_at: "2026-10-17T01:10:57.978370312+00:00"
id: "atelier-uv5d"
issue_type: "feature"
labels:
- "cli-output"
- "review"
priority: "P1"
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
closed_at: "2026-10-23T14:35:49.254342441+00:00"
status: "done"
title: "Show review state in review list <id>, with findings under dispositions"
updated_at: "2026-10-23T14:35:49.254342441+00:00"
---

## Goal

To answer "is atelier-bw0b approved? stale? which findings are still open?", a reader has to match 22 findings to 22 dispositions by eye across 45 unordered `review list` lines, then run `merge --check`, which only works with the work branch checked out. The list never says whether the latest approval (atelier-x16l @e2562bd6) is current or stale, or which findings still need action. `merge --check --json` already computes `supports` (approval and disposition IDs), but no text view shows it.

## Done when

`atelier review list <id>` starts with a state block, then shows findings as a tree:

```
Approval: ✓ atelier-x16l @e2562bd6 (Independent reviewer 3) current
          (or ⚠ stale since 1a2b3c4 "Tweak…": changed proof-sample.txt)
Findings: 22 · 21 resolved · 1 open
  ⚠ atelier-4cid defer, no disposition  The Stop hook runs at the end of every turn…
  ✓ atelier-1ia7 defer → fix_now atelier-g6fd @a485ef02  docs/adr/0022…:41-43 'prime … is removed'…
Other events: 3 review accounts (atelier-4dbb, atelier-dgmt, atelier-qdg9)
```

- A superseded disposition shows the chain (`no_action atelier-ul3r → no_action atelier-6o75`).
- Approval state comes from the same code as the `review.current` gate (crates/atelier-app/src/review_gate.rs), not a copy of it.
- The view works from any branch that has the records.
- A fixture test covers approved, stale, open-finding and superseded-disposition states.

## Context

Depends on the row format from the review-list item. The stale reason comes from the "which approval merge --check considered" item. atelier-sdb0 shows recent reviews in status, and this is the full view behind it. In transcripts, 4 of 5 `review list`/`review show` runs were piped to `head`.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. Build from review_gate::approval_state (review_gate.rs:314) and approval_stale_edges (:235), as status does (git.rs:1045); review_request.rs HistoryLine already pairs findings with dispositions. Show stale edges, not atelier-cisa's reason text (later increment atelier-imlh).
- 2026-10-23 [decision] The approval state is judged on the owner's work branch while it holds commits the base does not (review_gate::approval_standing, a new wrapper approval_state now delegates to, plus approval_stale_edges); otherwise, as for merged work, it is the newest approval review_gate::approval_accepted accepts on the base.
  Reason: approval_state on the base reports every merged approval as stale, because the fork point is HEAD there; approval_accepted is land's judgement of history.
  Rejected: approval_state everywhere; judging against HEAD of whatever branch is checked out
- 2026-10-23 [decision] Findings are counted disposed or open, open meaning no disposition, and marked ⚠ only when open; the view does not claim findings.disposed would accept a disposition.
  Reason: Acceptance needs the gate's full findings.disposed logic (approval pins, author identities, history); reimplementing it would copy gate code, and why a gate refuses belongs to atelier-imlh.
  Rejected: A 'resolved' count claiming gate acceptance
- 2026-10-23 [decision] review list <id> prints the event rows first, then the Approval line and the findings tree; there is no 'Other events' summary line.
  Reason: The increment Goal orders rows then state, and the rows already list every non-finding event.
  Rejected: Replacing the rows with the state block when an ID is given

## Close Reason

`atelier review list <id>` follows its rows with the owner's approval state (`✓ current`, `⚠ stale` with each commit that staled it and its paths, or `⚠ missing` on the work branch; `✓ accepted on <base>` / `⚠ not accepted on <base>` for merged work) and its findings, open ones first, each with its full disposition chain including superseded dispositions.

### Deviations
Findings are counted `disposed`/`open` rather than `resolved`, and there is no `Other events` line because the rows already list those events. The format follows the output contract (no brackets, `by <identity>`) rather than the item's sketch.

### Decisions
See the decision notes on this item (branch versus base judgement via approval_standing/approval_stale_edges/approval_accepted; disposed versus open; rows then state). `review_gate::approval_standing` was added and `approval_state` delegates to it; `independent` is now pub. Gate behaviour is unchanged.

### Surprises
`approval_state` cannot judge merged work: on the base the fork point is HEAD, so every approval reads as stale. Hence the base judgement.

### Open items
Why an approval is not current is atelier-cisa's (increment atelier-imlh).
