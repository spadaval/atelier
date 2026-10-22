---
created_at: "2026-10-17T19:00:36.352316978+00:00"
id: "atelier-mbrf"
issue_type: "feature"
labels:
- "land"
- "merge"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T23:19:39.707021115+00:00"
status: "done"
title: "Let a trivial change land on the base with a visible exemption, without an issue or work branch"
updated_at: "2026-10-22T23:19:39.707021115+00:00"
---

## Goal

A trivial change to a non-record file, such as a one-line `workflow.yaml` fix or a typo in an ADR, today needs an owner issue, a work branch, a `review trivial` event and `atelier merge`. `review trivial` pins a commit on a work branch, and `land` on the base accepts only record and strategy commits or commits covered by a merged owner's approval or exemption (`docs/product/reviews-and-merge.md`, `land_base_unreviewed`). So the exemption meant for changes whose correctness is evident costs a full cycle. Under the 2026-10-22 decision, review has one standard: non-trivial changes, prose included, get independent review, and only the trivial exemption skips it. This item removes the owner requirement from that exemption, and nothing else.

## Done when

- A trivial exemption can pin a base commit with no owner issue (for example `atelier review trivial --base <commit> --identity <who> "why correctness is evident"`), and `land` on the base counts that commit as covered.
- `land` still refuses an unexempted non-record commit with `land_base_unreviewed`, and an exemption pinned to one commit does not cover a later one.
- The exemption shows in the review history and in mission debriefs, as owner exemptions do today.
- `docs/product/reviews-and-merge.md` states the rule.
- Tests cover an exempted base commit, an unexempted one, and a later commit after an exemption.

## Context

The human rejected two alternatives on 2026-10-22: exempting planning paths from review, and a separate review standard for planning text. Records and mission strategies stay exempt as they are, because requiring an issue to edit issues would be recursive.

## Notes

- 2026-10-22 [note] From review of atelier-r28p (finding atelier-kotb): docs/product/reviews-and-merge.md:85-94, the comment at review_gate.rs:134-139 and atelier-r28p's decision note say a record 'filed on the base after the approval' stays editable; the code checks only that the record is absent from the approval's pinned tree, so a base record filed before the approval and merged in later is editable too. Reword all three to 'not in the approval's pinned tree' while editing this doc section.
- 2026-10-22 [decision] A base exemption is a trivial_exemption review file that names base: <branch> instead of target, loaded as its own BaseExemption type beside the review events; land and start count exactly its pinned commit's paths as covered, never its done claims or another commit
  Reason: The record keeps one exemption kind under one standard and stays visible in .atelier/reviews, review list and review show, while a separate type keeps owner gate code (review.current, approval_accepted, scope) from ever treating an exemption with no owner as an owner event. Old binaries cannot read the new field, so reinstall after merging.
  Rejected: An optional target on ReviewRecord (every gate site would need a no-owner case); requiring a mission or issue as target (an owner in all but name); a new event kind (a second exemption standard)
- 2026-10-22 [decision] A mission debrief shows the base exemptions recorded from the mission's start to its done claim on its timeline, so a base exemption shows in every mission running when it was recorded
  Reason: A base exemption names no mission; time is the only link that needs no extra argument, and showing it to every concurrent mission errs toward visibility.
  Rejected: An optional mission argument on review trivial --base (omitting it would hide the exemption from every debrief)

## Close Reason

`atelier review trivial --base <commit> --identity <who> "why"` records a trivial exemption of one base commit with no owner issue or work branch; `land` and `start` on the base count exactly that commit as covered, still refuse unexempted and later commits with `land_base_unreviewed` and an exempted done claim with `land_base_done_claim`, and the exemption shows in `review list`, `review show` and mission debriefs. `docs/product/reviews-and-merge.md` states the rule under "Trivial changes on the base".

### Deviations
The finding atelier-kotb wording fix was applied in reviews-and-merge.md and at allowed_edge, and as a correction note on atelier-r28p rather than an edit of its decision note, which stays as recorded. The positional arguments of `review trivial` became one list of one or two values (owner and reason, or the reason alone with `--base`), because clap's optional-leading-positional mode misparsed `trivial <id> --identity X "why"`; existing invocations are unchanged.

### Decisions
Storage and finding (base: <branch> instead of target, its own BaseExemption type, exact-commit coverage, no done-claim coverage) and debrief attribution (timeline window from the mission's start to its done claim): see this item's decision notes. Recording requires a checkout of the base branch and a commit the base already holds.

### Surprises
Older atelier binaries reject a review file with no target, so the installed CLI must be reinstalled after this merges and before any base exemption is committed.

### Open items
The vendored Agent Factory reference (`.agents/skills/agent-factory/references/atelier.md`) gained one sentence on `--base`; it is not yet upstream in spadaval/.agents.
