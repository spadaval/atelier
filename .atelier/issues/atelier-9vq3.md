---
created_at: "2026-10-15T22:08:58.961605084+00:00"
id: "atelier-9vq3"
issue_type: "task"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-9vq3
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-28mz"
    type: "related"
  - kind: "issue"
    id: "atelier-e4c5"
    type: "related"
  - kind: "issue"
    id: "atelier-o6h9"
    type: "related"
  - kind: "issue"
    id: "atelier-ymmb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-15T23:04:42.342135003+00:00"
status: "done"
title: "Decide whether epic should become a parent issue"
updated_at: "2026-10-15T23:04:42.342135003+00:00"
---

## Goal

Atelier's epic means "a unit of work that goes through the lifecycle together": it owns one branch and merges only when every child is done. Most trackers use epic for a theme ("Trade"), so adopters model themes as epics and get branches that never merge. astromech had to split its backlog into 19 increments for that reason.

## Done when

Sumeet decides between: (a) drop the epic type; any issue may have children, the parent owns the branch and can't close until its children are terminal (one level deep), and the epic body sections (Unknowns, Design) become optional sections on any issue; (b) keep the concept and rename it (slice, increment, batch); (c) keep it as is and document the difference. The decision is recorded in the product docs and the follow-up work is filed.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Recommendation: (a). Epic's Outcome/Proof/Delivers map onto an issue's Goal/Done when plus its children, so the separate type mostly adds a word that means something else elsewhere. Themes go to labels, and missions keep the why. It also removes the cases in atelier-28mz (children of draft epics), atelier-e4c5 (an epic's blockers and its children) and the ready-list noise, since one rule covers parents of any type. Migration: retype epics, keep their links.

## Notes

- 2026-10-15 [human] Sumeet chose (a): drop the epic type. Any non-mission issue may have children, one level deep; the parent owns the branch and cannot close until its children are terminal; Unknowns and Design become optional sections on any issue; themes go to labels and missions keep the why; existing epics are retyped and keep their links. Recorded in docs/product/work-model.md (Decided changes). Implementation filed as atelier-o6h9.

## Close Reason

Decided by Sumeet 2026-10-15: option (a). Drop the epic type; any non-mission issue may be a parent that owns its children's branch, one level deep, and cannot close until its children are terminal. Unknowns and Design become optional sections on any issue; themes are labels; epics are retyped keeping their links. Recorded in docs/product/work-model.md; implementation is atelier-o6h9.
