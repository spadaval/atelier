---
created_at: "2026-06-11T00:06:58.518089056+00:00"
id: "atelier-onba"
data: "{\"constraints\":[\"Mission Control must use first-class mission records and typed links, not infer missions from issue labels or parent trees except through documented compatibility behavior.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"A UI built before projection semantics stabilize could duplicate state logic or obscure stale projection and validation problems.\"],\"validation\":[\"Linked issues prove JSON projection schema, mission progress, blockers, evidence gaps, workflow health, TUI fixture coverage, and Milestone 6 closeout validation.\"],\"work\":[]}"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0003"
  - kind: "issue"
    id: "atelier-0006"
  - kind: "issue"
    id: "atelier-000c"
  - kind: "issue"
    id: "atelier-001o"
  - kind: "issue"
    id: "atelier-001x"
  - kind: "issue"
    id: "atelier-001y"
  attachments: []
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Mission Control projection and UI"
updated_at: "2026-06-11T19:54:27.193778212+00:00"
---

Superseded by mission atelier-n8ag, Autonomous mission operations.

Original intent preserved: expose durable Atelier state through mission-control surfaces. The scope is narrowed for now: Mission Control means CLI mission status/control, not a separate JSON projection or TUI.

The executable status/control work is now linked directly to atelier-n8ag so agents have one durable mission authority for autonomy-core work.
