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
status: "in_progress"
title: "Read review state per item without opening records"
updated_at: "2026-10-23T14:15:02.400982884+00:00"
---

## Goal

An agent can read an item's review state without opening record files: `review list <id>` shows one line per event in time order, then the approval state (current, stale with the edges that staled it, or missing) and findings with their dispositions beneath; a child lists its owner's reviews; `review show` is a brief Markdown view; recording any review event confirms what was recorded and warns when the identity is one of the code's authors; review titles do not repeat the body.

## Done when

CLI tests pin `review list <id>` (rows, state block, finding tree, child fallback, empty case), `review show` for a finding and a disposition, each recording command's confirmation line and the author-identity warning, a new record whose title differs from its body, and the shared invalid-value form for `--commit`, `--retention-days` and `--limit`. Delivers S4. Author proof plus independent review.

## Scope

Children: atelier-h6o4 (first; others reuse its row renderer, including `status.rs:323`), atelier-3u2u, atelier-093h, atelier-jw1n, atelier-hulj, atelier-uv5d, atelier-d8ku. uv5d shows staleness from `review_gate::approval_stale_edges`; the per-candidate reason text is atelier-cisa's, in a later increment. Reuse `review_gate` functions (`approval_state`, `author_identities`, `independent`) rather than copying them. Existing review records are not rewritten.

## Context

Regroom 2026-10-23 notes on each child. Code: `crates/atelier-cli/src/commands/review.rs` (`list` :139-170 uses `{:?}`, `show` :204 prints JSON, `append` :68), `crates/atelier-records/src/review.rs:266,298` (title = summary). 1147 of 1149 review records have title equal to body.
