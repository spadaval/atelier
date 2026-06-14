---
created_at: "2026-06-14T21:45:23.780062904+00:00"
id: "atelier-1cib"
issue_type: "task"
labels:
- "agent-factory"
- "template"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update subagent assignment template for epic branch review"
updated_at: "2026-06-14T21:45:23.780062904+00:00"
---

## Description

Revise the Agent Factory worker assignment block so workers receive mission workspace, epic branch, issue slice, proof destination, and independence requirements. Outcome: prompts stop requiring independent review for every issue and instead flag epic/mission review requirements. Evidence: template diff and example assignment review.

## Outcome

- Agent Factory worker assignment template includes mission workspace and parent epic branch fields.
- Assignment template distinguishes issue-local proof from epic or mission independent review requirements.
- Template tells workers when extra workspace isolation is justified.

## Evidence

- `git diff` transcript for the assignment template shows mission worktree, epic branch, assigned issue, proof destination, and independence fields.
- Example assignment review artifact demonstrates the new template on this mission.
- Targeted `rg` transcript proves old default independent-review wording for every issue is not present.
