---
created_at: "2026-10-23T14:13:37.603557648+00:00"
id: "atelier-phfo"
issue_type: "feature"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-phfo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-093h"
  - kind: "issue"
    id: "atelier-3u2u"
  - kind: "issue"
    id: "atelier-d8ku"
  - kind: "issue"
    id: "atelier-h6o4"
  - kind: "issue"
    id: "atelier-hulj"
  - kind: "issue"
    id: "atelier-jw1n"
  - kind: "issue"
    id: "atelier-uv5d"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T14:45:38.946961513+00:00"
status: "done"
title: "Read review state per item without opening records"
updated_at: "2026-10-23T14:45:38.946961513+00:00"
---

## Goal

An agent can read an item's review state without opening record files: `review list <id>` shows one line per event in time order, then the approval state (current, stale with the edges that staled it, or missing) and findings with their dispositions beneath; a child lists its owner's reviews; `review show` is a brief Markdown view; recording any review event confirms what was recorded and warns when the identity is one of the code's authors; review titles do not repeat the body.

## Done when

CLI tests pin `review list <id>` (rows, state block, finding tree, child fallback, empty case), `review show` for a finding and a disposition, each recording command's confirmation line and the author-identity warning, a new record whose title differs from its body, and the shared invalid-value form for `--commit`, `--retention-days` and `--limit`. Delivers S4. Author proof plus independent review.

## Scope

Children: atelier-h6o4 (first; others reuse its row renderer, including `status.rs:323`), atelier-3u2u, atelier-093h, atelier-jw1n, atelier-hulj, atelier-uv5d, atelier-d8ku. uv5d shows staleness from `review_gate::approval_stale_edges`; the per-candidate reason text is atelier-cisa's, in a later increment. Reuse `review_gate` functions (`approval_state`, `author_identities`, `independent`) rather than copying them. Existing review records are not rewritten.

## Context

Regroom 2026-10-23 notes on each child. Code: `crates/atelier-cli/src/commands/review.rs` (`list` :139-170 uses `{:?}`, `show` :204 prints JSON, `append` :68), `crates/atelier-records/src/review.rs:266,298` (title = summary). 1147 of 1149 review records have title equal to body.

## Close Reason

### Outcome
An item's review state reads without opening records. `review list <id>` prints one row per event in time order (kind, pin, identity, date, first summary line), then the approval state (current, stale with the commits and paths that staled it, missing, or accepted on the base) and findings with their disposition chains; a child shows its owner's view; an owner with no events says so (atelier-h6o4, atelier-uv5d, atelier-jw1n). `review show` is a brief Markdown view (atelier-3u2u). Recording an event prints one confirmation line and warns when the identity matches a Git author (atelier-093h). New review titles are generated from kind, owner and pin (atelier-hulj). `--commit`, `--resolution-commit`, `--retention-days` and `--limit` use the shared invalid-value form (atelier-d8ku). Delivers atelier-rc8j S4; shown by CLI tests in review_views.rs (receipt atelier-su46 and focused receipts on each child). Independent review compared `review list` with land and `merge --check` across eleven states and found no disagreement.

### Deviations
Rows come before the state block, following the increment goal rather than atelier-uv5d's example. `--json` is unchanged.

### Decisions
Decision notes on atelier-uv5d, atelier-h6o4, atelier-093h, atelier-jw1n, atelier-3u2u, atelier-hulj and atelier-d8ku; title format per atelier-rc8j's decision.

### Surprises
None material. The author warning is best-effort: from the base checkout there are no branch authors to compare (finding atelier-p28x, no_action).

### Open items
The reason an approval is not current is atelier-imlh. Mission decisions listed twice in `issue show --full` is atelier-x0vs.
