---
created_at: "2026-06-11T19:53:21.659407023+00:00"
id: "atelier-n8ag"
data: "{\"constraints\":[\"Atelier is the durable coordination substrate for Agent Factory mission work.\",\"Mission guidance lives in CLI workflows and Agent Factory procedures, not a separate Mission Control UI yet.\",\"The repository has at most one active mission focus; parallel worktrees inherit or explicitly switch that focus.\",\"Highly consequential decisions are resolved before mission start; mission-linked decision issues are limited to local execution choices that do not undermine autonomy.\",\"Non-mission issue tasks stay lightweight; high-risk mission work can require evidence, validators, and waivers.\",\"Human-first command output remains the agent interface; do not restore command-result JSON.\",\"Existing Markdown-first state and rebuildable projection boundaries remain intact.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Over-strict workflow policy could make ordinary agent work feel like red tape, while under-specified policy could let closeout quality drift.\",\"Guidance-only process can be skipped by humans or agents unless important gates are enforced by workflow validators and lint.\",\"Agent Factory guidance can drift from Atelier CLI behavior unless the skill and CLI are validated together.\",\"Missions can lose autonomy if high-consequence product, architecture, persistence, security, or public-contract decisions are left unresolved inside the mission graph.\",\"A separate dashboard or projection built too early could duplicate state logic before CLI mission status proves the needed control surface.\"],\"validation\":[\"Linked work proves lightweight and strict workflows, validator-backed transitions, waivers, action-aware guidance, mission status CLI, evidence gaps, blocker visibility, active mission focus, Agent Factory guidance, branch/worktree ergonomics, decision readiness, and closeout proof.\"],\"work\":[]}"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0001"
  - kind: "issue"
    id: "atelier-0006"
  - kind: "issue"
    id: "atelier-0007"
  - kind: "issue"
    id: "atelier-0008"
  - kind: "issue"
    id: "atelier-000c"
  - kind: "issue"
    id: "atelier-000i"
  - kind: "issue"
    id: "atelier-000l"
  - kind: "issue"
    id: "atelier-000r"
  - kind: "issue"
    id: "atelier-001m"
  - kind: "issue"
    id: "atelier-001p"
  - kind: "issue"
    id: "atelier-001q"
  - kind: "issue"
    id: "atelier-001r"
  - kind: "issue"
    id: "atelier-zh61"
  attachments:
  - kind: "evidence"
    id: "atelier-kpp5"
    role: "validates"
  - kind: "evidence"
    id: "atelier-rodj"
    role: "validates"
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Autonomous mission operations"
updated_at: "2026-06-11T21:20:37.872775104+00:00"
---

Make `atelier` the orchestrator's best friend for highly autonomous,
goal-directed agent work. An orchestrator should be able to focus one active
mission, see what should happen next, delegate ready issue slices, create or
locate isolated branches/worktrees, run and enforce validations, ensure required
evidence is present, see blockers and closeout gaps, and finish with durable
proof. The system should enforce the gates humans and agents commonly skip while
keeping ordinary non-mission issue work lightweight.
