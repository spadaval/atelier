---
created_at: "2026-10-23T00:10:58.567803052+00:00"
id: "atelier-3js6"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-16sz"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-30T06:41:06.901819130+00:00"
status: "superseded"
title: "Add a 'just a doc' pressure scenario to Agent Factory's skill evaluation"
updated_at: "2026-10-30T06:41:06.901819130+00:00"
---

## Goal

After atelier-aufe the skill requires independent review of non-trivial prose, but maintainers/skill-evaluation.md (and the upstream evals in spadaval/.agents evals/agent-factory) have no scenario where an agent calls a non-trivial doc, ADR or plan change 'just a doc' and skips review, so a regression would go unnoticed. Found in review of atelier-aufe (finding atelier-16sz).

## Done when

A pressure scenario with that rationalization exists in the skill evaluation fixtures and upstream evals, run with and without the aufe wording as the method describes, with results recorded.

## Close Reason

The 'It's just docs' pressure scenario is in the skill's maintainers/skill-evaluation.md (spadaval/.agents a2c1050, row "It's just docs"). Skill evaluation work belongs to spadaval/.agents (ADR 0023); no run with and without the wording is recorded here.
