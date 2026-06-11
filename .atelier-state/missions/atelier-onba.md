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
status: "open"
title: "Mission Control projection and UI"
updated_at: "2026-06-11T00:06:58.518089056+00:00"
---

Expose durable Atelier state through deterministic Mission Control projections and an optional read-only terminal UI that consumes those projections instead of inventing a separate state model.
