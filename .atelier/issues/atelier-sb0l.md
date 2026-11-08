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
status: "in_progress"
title: "Add a groomer: re-sequence the backlog across missions, and bring the human what we should not do"
updated_at: "2026-11-08T22:43:16.709467282+00:00"
---

## Goal

Give Agent Factory a groomer (`procedures/groom.md`) that keeps the backlog worth working from. It re-sorts and re-sequences issues into and across missions, and finds work we should not do: stale, duplicated, unready, or costing more in scope, complexity or risk than it is worth. The agent does the bulk reading and plain record fixes; the human makes the planning calls from one digest.

## Done when

- [ ] Scope and inputs: a run covers one repository, either its whole open backlog or one area (a mission, label or set of items). It reads the open items, missions and their strategies, deferred review findings, and the code and git history it needs to check an item still makes sense. External queues (dependency PRs) and judging work already merged (review's verdict, the gardener) are out of scope.
- [ ] Sequencing: groom.md has the run propose moving items into, out of and across missions, forming a mission from a cluster of unassigned items, splitting or merging items, reordering work and fixing blockers, and changing priorities, each with its reason.
- [ ] Not doing: the run flags items that are stale (done by other work, or naming code or decisions that no longer exist), duplicated, or not worth their cost. Cost is sized as complexity, value and risk on review.md's three levels with its anchors, so a groomed item and a reviewed change are judged on one scale. Each flag proposes `decline` or `supersede` with the reason.
- [ ] Shape: the run flags items that cannot be started as written (no checkable Done when, too big for one review, or waiting on a human decision no item tracks) and proposes the rewrite, split or decision item.
- [ ] Authority: the agent applies only record fixes whose correctness is evident from the records (a missing label, a duplicate link) and lists them in its report. Declines, supersedes, re-sequencing and mission moves go to the human as one digest per run and repository: an item labelled `needs-human` whose entries each give the item, the evidence, the proposed change and its size. The human answers each entry in a note. The next run applies accepted entries, adds to an unanswered digest instead of filing another, and does not re-propose a rejected entry without new evidence.
- [ ] Triggers: groom.md says when to run it (when a mission closes, before planning a mission, when the human asks). plan.md's "Swept backlog first" rule points at groom scoped to the area being planned, so the skill keeps one version of the sweep.
- [ ] SKILL.md lists `groom`. The agent-facing skill stays within atelier-qj9d's limits, counted by its rule: at most 1,000 lines, SKILL.md at most 100, groom's read path at most 300. The Report gives the counts before and after.
- [ ] Trial: a fresh agent given only the skill runs groom on this repository's whole backlog, and its output is read by hand. Each stale or duplicate claim is checked against the records and code, and its digest is in the Report. The trial applies, files and merges nothing.

## Context

Agreed with Sumeet in conversation on 2026-11-08. plan.md already has a "Swept backlog first" rule (atelier-l58v), but nothing looks at backlog outside an area about to be planned: 52 items belong to no mission, and the draft missions atelier-4pzj and atelier-uf97 hold 60 more in backlog. atelier-3sk9 moved backlog sweeping out of the gardener because it is product work; this is where it went. The skill change is made in a worktree of spadaval/.agents (`~/projects/agents-skills`) and merged to its main when this item merges; if atelier-dkul has moved the skill here first, the change moves with it. Three implementers (Opus, Grok, Sol) draft it independently for comparison; one draft, or a merge of them, goes to review.
