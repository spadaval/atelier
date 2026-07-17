---
created_at: "2026-07-06T20:37:49.381094280+00:00"
id: "atelier-wlk4"
issue_type: "task"
labels:
- "docs"
- "mission-review"
- "planning-standards"
- "subskill-docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2uim"
  - kind: "issue"
    id: "atelier-amfw"
  - kind: "issue"
    id: "atelier-l5mw"
  - kind: "issue"
    id: "atelier-wyxn"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-09T16:26:13.518884051+00:00"
status: "done"
title: "Publish mission and issue-set authoring standards"
updated_at: "2026-07-09T16:26:13.518884051+00:00"
---

## Description

Assigned subskill: docs. Consolidate the distributed planning rules into one authoritative standard and route Agent Factory and repository documentation to it. Cover mission, epic, executable issue, validation issue, and evidence responsibilities; outcome and non-scope quality; complete outcome-to-work coverage; coherent integration boundaries; issue accountability; internal and external dependency semantics; transitive execution readiness; unresolved decision work; migration, documentation, cleanup, and closeout coverage; initial parallel-work safety; and concise positive and negative examples.

## Outcome

- A fresh planner or reviewer can determine whether a mission issue set is complete, correctly decomposed, correctly sequenced, and independently reviewable without private chat history.
- The standard includes a normative mission-review checklist and examples of missing external blockers, unsafe initial parallelism, orphaned outcome claims, duplicate ownership, and well-formed issue graphs.
- Existing planning and validation guidance links to the standard and does not retain contradictory duplicate rules.

## Evidence

- A fresh planner uses the published checklist to classify one complete mission graph and examples with a missing external blocker, unsafe initial parallelism, orphaned Outcome claim, duplicate ownership, and malformed validation coverage; the artifact records the affected issue IDs and dependency paths.
- A fresh mission reviewer can follow links from Agent Factory and the repository documentation to the single normative standard without private context, and focused searches show contradictory duplicate planning rules were removed or explicitly routed to it.
- The documentation diff, `git diff --check -- '*.md'`, and `atelier check atelier-wlk4` are captured on this issue; process-policy judgment is independently covered by atelier-t876.
