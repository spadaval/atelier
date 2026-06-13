---
created_at: "2026-06-13T02:36:08.361649921+00:00"
id: "atelier-dxy1"
issue_type: "task"
labels:
- "agent-factory"
- "assignee:root"
- "delegation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-29yn"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T04:19:27.423552685+00:00"
status: "done"
title: "Update delegation prompt template for bounded proof slices"
updated_at: "2026-06-13T04:19:27.423552685+00:00"
---

## Description

Revise delegation guidance so subagents receive narrow, evidence-producing assignments earlier in the mission. The template should reduce private context and avoid main-agent over-collection.

## Outcome

- Worker assignments require one role, exact tracker IDs, owned files or workflows, out-of-scope boundaries, expected proof, evidence destination, independence requirement, and model rationale.
- Guidance encourages early scout/audit/validation subagents for bounded slices.
- Delegation handoffs produce evidence or tracker updates, not only prose summaries.
- Final handoffs use a compact required schema: result, issue ID, subskill,
  changed files, evidence IDs, commands run, dirty state, branch or commit,
  blockers, and exact follow-up recommendation.

## Evidence

- File-change review of Agent Factory procedure docs shows the updated template.
- Review artifact compares at least one weak atelier-tcmr delegation pattern with the revised template and explains which required fields would have exposed the proof gap earlier.
- Dogfood transcript or review shows a subagent handoff using the required
  schema and requiring no private chat mining for closeout.
- `atelier lint`, `atelier export --check`, and docs whitespace check commands pass.
