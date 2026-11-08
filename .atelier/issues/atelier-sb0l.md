---
created_at: "2026-11-08T22:43:12.358093041+00:00"
id: "atelier-sb0l"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-sb0l
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T23:57:48.771539137+00:00"
status: "done"
title: "Add a groomer: re-sequence the backlog across missions, and bring the human what we should not do"
updated_at: "2026-11-08T23:57:48.771539137+00:00"
---

## Goal

Give Agent Factory a groomer (`procedures/groom.md`) that keeps the backlog worth working from. It re-sorts and re-sequences issues into and across missions, and finds work we should not do: stale, duplicated, unready, or costing more in scope, complexity or risk than it is worth. The agent does the bulk reading and plain record fixes; the human makes the planning calls from one digest.

## Done when

- [x] Scope and inputs: a run covers one repository, either its whole open backlog or one area (a mission, label or set of items). It reads the open items, missions and their strategies, deferred review findings, and the code and git history it needs to check an item still makes sense. External queues (dependency PRs) and judging work already merged (review's verdict, the gardener) are out of scope.
- [x] Sequencing: groom.md has the run propose moving items into, out of and across missions, forming a mission from a cluster of unassigned items, splitting or merging items, reordering work and fixing blockers, and changing priorities, each with its reason.
- [x] Not doing: the run flags items that are stale (done by other work, or naming code or decisions that no longer exist), duplicated, or not worth their cost. Cost is sized as complexity, value and risk on review.md's three levels with its anchors, so a groomed item and a reviewed change are judged on one scale. Each flag proposes `decline` or `supersede` with the reason.
- [x] Shape: the run flags items that cannot be started as written (no checkable Done when, too big for one review, or waiting on a human decision no item tracks) and proposes the rewrite, split or decision item.
- [x] Authority: the agent applies only record fixes whose correctness is evident from the records (a missing label, a duplicate link) and lists them in its report. Declines, supersedes, re-sequencing and mission moves go to the human as one digest per run and repository: an item labelled `needs-human` whose entries each give the item, the evidence, the proposed change and its size. The human answers each entry in a note. The next run applies accepted entries, adds to an unanswered digest instead of filing another, and does not re-propose a rejected entry without new evidence.
- [x] Triggers: groom.md says when to run it (when a mission closes, before planning a mission, when the human asks). plan.md's "Swept backlog first" rule points at groom scoped to the area being planned, so the skill keeps one version of the sweep.
- [x] SKILL.md lists `groom`. The agent-facing skill stays within atelier-qj9d's limits, counted by its rule: at most 1,000 lines, SKILL.md at most 100, groom's read path at most 300. The Report gives the counts before and after.
- [x] Trial: a fresh agent given only the skill runs groom on this repository's whole backlog, and its output is read by hand. Each stale or duplicate claim is checked against the records and code, and its digest is in the Report. The trial applies, files and merges nothing.

## Context

Agreed with Sumeet in conversation on 2026-11-08. plan.md already has a "Swept backlog first" rule (atelier-l58v), but nothing looks at backlog outside an area about to be planned: 52 items belong to no mission, and the draft missions atelier-4pzj and atelier-uf97 hold 60 more in backlog. atelier-3sk9 moved backlog sweeping out of the gardener because it is product work; this is where it went. atelier-dkul moved the skill into this repository while the drafts were written (the spadaval/.agents copy is frozen), so the change is made here in `skills/agent-factory`; the drafts' base was identical to the moved copy. Three implementers (Opus, Grok, Sol) draft it independently for comparison; one draft, or a merge of them, goes to review.

## Report

`procedures/groom.md` is new. A run reads one repository's open backlog (or one area and the items around it), applies only record fixes evident from the records, and puts every planning call in one digest: a `decision` item labelled `needs-human` and `groom`, closed with `decide` once every entry is answered and applied. Review's complexity, value and risk scales moved unchanged into `references/sizing.md`, which both review.md's Verdict and groom link, so they share one scale. plan.md's "Swept backlog first" now runs groom on the mission's area. SKILL.md lists `groom`.

Drafts: Opus, Sol and Grok 4.6 each drafted it independently in worktrees of spadaval/.agents (`feature/atelier-sb0l-{opus,sol,grok}`). Opus and Sol each moved the scales into a shared reference and made the digest a decision item. Grok restated the scales inside groom.md and used a task digest that cannot finish. The merge takes Opus's text, Sol's recheck of accepted entries before applying them and its reading of neighbours on an area run, and Grok's whole-backlog default.

Line counts (atelier-qj9d's rule, Markdown only):

| | Before | After | Limit |
|---|---|---|---|
| Agent-facing skill | 872 | 957 | 1,000 |
| SKILL.md | 96 | 97 | 100 |
| groom's read path (SKILL.md, groom.md, sizing.md, atelier.md) | n/a | 297 | 300 |
| review's read path | 318 | 323 | 300 |
| plan's read path | 302 | 302 | 300 |

Review's and plan's paths were already over 300 before this change; review's grows by 5 because its Verdict now links sizing.md (14 lines) instead of holding the scales (12 lines inline). Shortening them is not this item's scope.

Trial: a fresh Opus agent given only the skill ran groom on the whole backlog at 87433db5 (125 open items), wrote its digest to a scratch file and changed nothing. I checked every stale and duplicate claim against the records and code; each held: atelier-wiri (2fd7620f repointed the four links), atelier-4gsb (review.md:99 and :106 ask for severity again, as review_request.rs:676 and :683 do), atelier-8ghm (the test was removed in bf4fddad), atelier-pg2b (atelier-nz6a's branch adds the install lines), atelier-4urj (`evaluate_issue_gate` is gone), atelier-b17k into atelier-tf9n (both change `apply_ready_execution_gates`), and the four related-link pairs (each names the other). Its digest, 21 entries:

- Sequence: G1 split atelier-4pzj's start-and-ready half into its own mission; G2 move 4pzj's nine P3 polish items out; G3 merge atelier-b17k into atelier-tf9n; G4 form a mission "No unreviewed code reaches master" from atelier-fad5, atelier-nl9a and atelier-1nlr; G5-G7 plan atelier-jjvc, atelier-hfme and atelier-fpxc, whose pick-up conditions now hold; G8 block the ten debrief items on atelier-jdpf; G9 answer atelier-3km4 before vt8q's records become prunable on 2026-11-11; G10 fold atelier-c6g2 into atelier-avx0 and atelier-fb1i.
- Not doing: G11 supersede atelier-wiri; G12 decline atelier-4gsb; G13 supersede atelier-8ghm; G14 supersede atelier-pg2b once atelier-nz6a merges; G15 decline atelier-4urj; G16 decline atelier-w0su (Sumeet's call; the skill already routes around integration branches); G17 decline atelier-e95f (low confidence).
- Shape: G18 rewrite atelier-2yef; G19 give atelier-rqzu a Done when and file a bug on `review defer`'s output; G20 file the decision atelier-k55o and atelier-w4ef wait on; G21 rewrite atelier-sl01 to its remaining half.
- Record fixes it would apply: four related links, the `agent-factory` label on four items, and boxing the bulleted Done when lines of 34 items.

The trial's three problems with the procedure are fixed in groom.md: it now says what to check first on a large backlog and that cheaper agents may do the bulk reading, with each claim checked before it becomes an entry; an entry may cover several items and gives a new body as an outline; and it covers parked items whose condition holds, partly done items, deadline entries listed first, and format fixes `atelier check` asks for as record fixes.
