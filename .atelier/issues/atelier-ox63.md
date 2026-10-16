---
created_at: "2026-10-15T22:08:59.029206534+00:00"
id: "atelier-ox63"
issue_type: "feature"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-ox63
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3mjv"
    type: "related"
  - kind: "issue"
    id: "atelier-q8ok"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T01:06:53.726212089+00:00"
status: "done"
title: "Give decisions a record shape"
updated_at: "2026-10-16T01:06:53.726212089+00:00"
---

## Goal

Design questions can be filed (atelier-28mz is a task titled "Decide …"), but nothing marks them as decisions. They have no Options or Decision section, and they show up in `work ready` like code work even though most need the human and no branch.

## Done when

A decision type (or a typed section on any issue) with Question, Options and Decision; closing requires the Decision to be filled; `work ready` and the needs-human marking (atelier-q8ok) treat decisions as the human's by default; they can close on the base branch under the planning-change rule (atelier-3mjv).

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Sumeet: there should be a way to file issues for design questions. Today they're filed as tasks.

## Close Reason

decision issue type (in atelier init's policy and this repo's) with Question (required), Options, Decision, Context and Background sections; its decide transition requires the Decision (decision.recorded). Open decisions classify as needs a human, so work ready never offers them to agents and work human lists them. They need no branch and close on the base branch as planning changes (atelier-3mjv). Documented in work-model.md, workflow-configuration.md, reviews-and-merge.md and CONTEXT.md. Test: test_decisions_wait_for_the_human_and_close_with_a_recorded_decision.
