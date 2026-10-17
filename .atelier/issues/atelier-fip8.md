---
created_at: "2026-10-17T17:25:58.881272712+00:00"
id: "atelier-fip8"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-fip8
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Record decisions as typed notes and show which decision records are current"
updated_at: "2026-10-17T17:37:55.843715712+00:00"
---

## Goal

S1 and S3 of docs/strategies/review-and-introspection.md (mission atelier-r6bt). Implementers make decisions inside work items, and the decisions are lost when the session ends. Recording one should take a single command, and views should be able to read the decision back as structured data, so `debrief` (S4) and `status` (S6) can build on it. Decision records (issue type `decision`) also need to name the ADR they produced and the earlier decision they replace.

## Done when

- `atelier issue note <id> --kind decision "<choice>" --reason "<why>"` appends a decision note. `--rejected "<alternative>"` (repeatable) is optional, and `--one-way` marks the decision irreversible. `--reason` is required for decision notes, and the other note kinds refuse the decision flags.
- `check` refuses a decision note that has no choice or no reason, naming the issue and the note.
- `atelier work decisions <mission>` lists the decisions across the mission's scope (the mission, its advancing items and their children): one line each, one-way decisions first, and decision records included.
- A decision record can name its ADR (`issue update --adr <path>`; `check` refuses a path that does not exist), and a `supersedes` link from a newer decision record marks the older one superseded. `work decisions` and `issue show` mark superseded decisions, and only current ones count as current.
- CLI integration tests cover each point. Product docs (work-model.md, cli-surface.md) and the Agent Factory skill's tracker guidance describe decision notes.

## Context

The note kinds today are note, plan, observation, blocker, resolution, result, handoff and human (`atelier issue note --help`). The `supersedes` link type already exists in WELL_KNOWN_LINK_TYPES. A mission's linked strategy path is the precedent for a checked path field.

## Design

Store a decision note as an ordinary dated note with structured continuation lines, so existing note parsing and approval freshness rules apply unchanged:

```
- 2026-10-17 [decision one-way] Store the report in Close Reason subsections.
  Reason: Close Reason is already exempt from approval freshness.
  Rejected: a new Report section; a sidecar file.
```

The tag is `decision` or `decision one-way`. One parser in atelier-records turns notes into typed entries; `check`, `work decisions` and later views share it.

## Notes

- 2026-10-17 [decision one-way] Store decision notes as ordinary dated notes tagged [decision] or [decision one-way], with indented Reason: and Rejected: lines
  Reason: Existing note parsing, approval freshness and record validation apply unchanged, and the stored text reads well without a tool
  Rejected: a new Decisions section in the issue body
  Rejected: a structured YAML list in front matter
- 2026-10-17 [decision one-way] Link a decision record's ADR with a typed adr front matter field, checked like a mission's strategy path
  Reason: check can refuse a missing file, and views read the link without parsing prose
  Rejected: an ADR line inside the Decision section
- 2026-10-17 [decision] Treat a decision record as superseded only once the record superseding it is decided
  Reason: An open proposal to replace a decision does not change what is in force
- 2026-10-17 [decision] Leave decision notes without supersession; only decision records supersede
  Reason: S3 names decision records, and reversing an item-level choice is a new decision note or record; add note supersession when a view shows the need
