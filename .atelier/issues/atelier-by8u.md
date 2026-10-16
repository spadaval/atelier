---
created_at: "2026-10-15T21:00:45.354133130+00:00"
id: "atelier-by8u"
issue_type: "feature"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-by8u
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r7cd"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Give missions and epics a place for research and references"
updated_at: "2026-10-16T00:49:30.826583903+00:00"
---

## Goal

Mission bodies accept only Outcome, Success criteria and Epics, and an epic's Design is meant to be a link. Research that motivates a mission, or applies to every child of an epic, has no home in a record. It ends up in a separate strategy file, or in one child issue's Context. Decide where shared research, references and motivation live in records, and support it.

## Done when

A mission and an epic can each carry free-form background (research, references, motivation) that `issue show` displays and `check` accepts, or a decision record explains the intended home and Agent Factory's tracker-records reference matches it.

## Context

astromech: the EDDN sender rules apply to six EDDN issues but had to go in the Context of "EDDN sender core". The tool-by-tool research table and the won't-do list went into `docs/strategies/replace-the-toolbox.md`. Parser: `crates/atelier-records/src/lib.rs` (allowed sections per type).

Found 2026-10-15 while moving astromech's BACKLOG.md (about 135 items with research, references and decisions) into Atelier records. Sumeet's principle: Atelier records should hold the whole backlog; needing a side document means Atelier needs work.
