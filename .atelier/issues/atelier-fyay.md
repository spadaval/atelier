---
created_at: "2026-11-09T00:49:56.415123409+00:00"
id: "atelier-fyay"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-fyay
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Have planners settle what could break a mission before it runs, using quick spikes"
updated_at: "2026-11-09T00:49:56.821648001+00:00"
---

## Goal

Missions are meant to run mostly unattended, but `plan.md` lets a mission start with unknowns that could change its outcome, shape or scope; when one bites, the mission stalls until the human revises the strategy. Change the planning guidance so a planner anticipates what could break the mission and settles those questions first, often with a quick spike, without requiring every unknown to be resolved.

## Done when

- [ ] `skills/agent-factory/procedures/plan.md` asks the planner what would most likely stall the mission or force a strategy change, and requires each likely cause that could change the outcome, shape or scope to be settled before the mission is ready, while route-only unknowns wait for their evidence boundary.
- [ ] plan.md says a question a quick spike can answer is usually settled that way: the planner runs a small spike itself, or files a larger one outside the mission and asks the human to approve running it, and records what the spike showed.
- [ ] plan.md's Ready Work test for a mission includes the confidence bar (expected to reach its outcome without the human's call on its strategy), and the strategy's Risks and Unknowns lists what is left open with what happens if each bites.
- [ ] The guidance stays a judgment call: no scoring, no new CLI check, and no rule that Unknowns be empty.

## Context

Sumeet raised this on 2026-11-08: agents file missions with big unknowns, but swinging to a rigid rule would not work either, since no plan survives first contact.
