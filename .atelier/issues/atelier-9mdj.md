---
created_at: "2026-11-08T05:51:19.282116207+00:00"
id: "atelier-9mdj"
issue_type: "feature"
labels:
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-l58v"
    type: "supersedes"
  - kind: "issue"
    id: "atelier-stnv"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T05:57:29.150279118+00:00"
status: "superseded"
title: "Record a review verdict, so a reviewer can stop a change that costs more than it buys"
updated_at: "2026-11-08T05:57:29.150279118+00:00"
---

## Goal

Reviewers find real defects but have no recorded way to say the change as a whole should not merge. Let a review carry a verdict (merge, shrink or reject) with sized complexity, value and risk, and let shrink and reject stop the merge until someone other than the author decides them.

## Done when

- [ ] A reviewer can record a verdict on an item: `merge`, `shrink` or `reject`, plus complexity, value and risk each `S`, `M` or `L`, and a justification. Each is required when a verdict is given. It is pinned to a commit like other review events (proposed: flags on `review record`).
- [ ] `atelier merge <id>` refuses while the latest verdict on the item is `shrink` or `reject` and has no decision, and the refusal names the verdict and the command that decides it.
- [ ] An open shrink or reject can be decided (proposed: `review disposition` on the verdict event): uphold it, or override it with a required reason. An override lets merge proceed. The decision records its identity; like finding dispositions, Atelier does not compare it with the Git authors (atelier-md0v), and the skill says the author does not decide.
- [ ] Upholding a reject moves the item to `declined` with the reason. Upholding a shrink leaves merge refused until a later verdict on a newer commit says `merge` or `shrink` is overridden.
- [ ] `review list`, `review show`, the review brief, `debrief` and `doctor` show the verdict and its sizes, and any decision on it.
- [ ] A repository whose records have no verdicts behaves as before: no verdict means merge is not blocked by this gate.
- [ ] Tests cover: a verdict with a missing size refused; merge refused under an open reject; override without a reason refused; override with a reason lets merge pass; an upheld reject declines the item; an older repository without verdicts merges unchanged.
- [ ] `docs/product` describes the verdict and its sizes (S/M/L anchors as `atelier-l58v` writes them for the skill).

## Scope

Atelier's code, `docs/product` and this item's records. Not the skill: the guidance that tells reviewers to use the command lands after the skill moves here (`atelier-7iq0`).

## Context

Agreed with Sumeet on 2026-11-08 in the review audit thread: the reviewer opens with merge, shrink or reject, then complexity, value and risk sized S/M/L with written anchors; shrink and reject block merge until the accountable agent (never the author) decides; overriding needs a recorded reason; a reject that stands ends the work. `atelier-stnv` adds won't-fix and the `declined` status but not the verdict, `atelier-l58v` records shrink and reject as `fix_now` findings whose text opens with the verdict, which needs no binary change; this item makes the verdict and its sizes a record of their own, and `atelier-7iq0` moves the skill onto it.

## Unknowns

- Whether the verdict is a field on `review record` or its own event. Choose the one that keeps `review --help` shortest; record the choice as a decision note.

## Report

Superseded by atelier-l58v. Sumeet chose on 2026-11-08 to record shrink and reject as fix_now findings whose text opens with the verdict, as l58v plans, with no new record or merge gate. The sizes live in the finding's text.
