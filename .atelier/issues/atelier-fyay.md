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
closed_at: "2026-11-09T01:20:03.243380053+00:00"
status: "done"
title: "Have planners settle what could break a mission before it runs, using quick spikes"
updated_at: "2026-11-09T01:20:03.243380053+00:00"
---

## Goal

Missions are meant to run mostly unattended, but `plan.md` lets a mission start with unknowns that could change its outcome, shape or scope; when one bites, the mission stalls until the human revises the strategy. Change the planning guidance so a planner anticipates what could break the mission and settles those questions first, often with a quick spike, without requiring every unknown to be resolved.

## Done when

- [x] `skills/agent-factory/procedures/plan.md` asks the planner what would most likely stall the mission or force a strategy change, and requires each likely cause that could change the outcome, shape or scope to be settled before the mission is ready, while route-only unknowns wait for their evidence boundary.
- [x] plan.md says a question a quick spike can answer is usually settled that way: the planner runs a small spike itself, or files a larger one outside the mission and asks the human to approve running it, and records what the spike showed.
- [x] plan.md's Ready Work test for a mission includes the confidence bar (expected to reach its outcome without the human's call on its strategy), and the strategy's Risks and Unknowns lists what is left open with what happens if each bites.
- [x] The guidance stays a judgment call: no scoring, no new CLI check, and no rule that Unknowns be empty.

## Context

Sumeet raised this on 2026-11-08: agents file missions with big unknowns, but swinging to a rigid rule would not work either, since no plan survives first contact.

## Report

plan.md gains a "Confident enough to run unattended" shaping test: the planner asks what would most likely stall the mission or force a strategy change, settles each such cause that could change the outcome, shape or scope, and leaves route-only unknowns for their evidence boundary. A quick spike is named as the usual way to settle such a question: the planner runs a small one itself, or files a larger one outside the mission and asks the human to approve it, and records what it shows. Ready Work's mission test now says "passes the shaping tests, confidence included", and Risks and Unknowns lists what is left open with what happens if each bites. No CLI or check changes.

Size (atelier-mt5r): the plan role's read path (SKILL.md + plan.md + references/atelier.md, counted whole) was already 302 lines on master (97 + 100 + 105), over atelier-qj9d's 300. The first version took it to 315. To pay for the new rule, plan.md's existing text is tightened without changing what it asks: the Layers opener no longer lists strategy contents the Strategy File section lists, and a few clauses are shortened (One theme, Finishable, Swept backlog, Success criteria, Strategy File's last sentence, Ready Work's approval command, Strategic Revision). plan.md stays at 100 lines, so the read path is back to master's 302: the change adds no net lines, and the 2-line overrun predates it. Getting under 300 would mean cutting rules or shared files, which is out of this item's scope. No tests cover skill prose; none were run.
