---
created_at: "2026-11-09T05:55:07.731261036+00:00"
id: "atelier-pxme"
issue_type: "task"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-lfkn"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make SKILL.md's ask-a-human rules agree with complete-or-abort inside a mission"
updated_at: "2026-11-09T05:55:18.069843733+00:00"
---

## Goal

atelier-n7c4 made the orchestrator complete a mission or abort it, with no human work inside it. Two lines on every role's read path still send a mid-mission question to the human: SKILL.md's Respect authority ("Ask a human about product direction, risk tolerance and destructive or irreversible actions policy does not authorize") and references/atelier.md's Records ("agents leave it, and open decisions, to the human"). Both reviewers of atelier-n7c4 named them. Inside a mission, such a question is either settled within the strategy and Authority or aborts the mission; outside a mission the rules stand.

## Done when

- [ ] SKILL.md's Respect authority and references/atelier.md's Records say, in as few words as possible, that inside an approved mission a question beyond the strategy and its Authority aborts the mission (as orchestrate.md says) rather than waiting on the human; outside a mission the existing rule is unchanged.
- [ ] No other sentence on the orchestrator's or implementer's read path tells them to ask, consult or wait for the human mid-mission; the Report quotes any that remain and says why each is about aborting, closing or work outside a mission.

## Scope

SKILL.md and references/atelier.md only, on atelier-ss4y's branch.
