---
created_at: "2026-06-10T03:51:07.274706851+00:00"
id: "atelier-m25t"
issue_type: "task"
labels:
- "assignee:root"
- "domain-model"
- "evidence"
- "mission"
- "plan"
- "record-store"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-a4ps"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T18:59:58.882617064+00:00"
status: "done"
title: "Extend Markdown-first storage to first-class records"
updated_at: "2026-06-10T18:59:58.882617064+00:00"
---

## Description

Prepare the Markdown-first storage boundary for first-class missions, milestones, plans, evidence, and workflow validator records.

Scope:
- Extend or generalize RecordStore and ProjectionIndex contracts beyond issues without implementing every command surface at once.
- Define record-kind registration, schema/version validation, link target validation, and deterministic rendering hooks for first-class records.
- Coordinate with existing open issues for mission, plan, evidence, milestone, workflow validator, and Mission Control work.
- Avoid reintroducing typed numeric IDs or aggregate manifest/graph canonical files.
First-class record kinds have an implementation-ready storage contract that reuses the Markdown-first architecture; issue-specific assumptions are isolated; future mission/plan/evidence command issues can depend on this slice rather than inventing separate persistence paths; tests or fixtures cover at least one non-issue record kind or a documented stub contract with validation.
- cargo fmt -- --check
- cargo test
- ./target/debug/atelier export --check
- ./target/debug/atelier doctor
- docs/index.md and docs/architecture/index.md remain consistent

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
