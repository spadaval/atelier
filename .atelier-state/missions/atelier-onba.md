---
created_at: "2026-06-11T00:06:58.518089056+00:00"
id: "atelier-onba"
data: "{\"constraints\":[\"Mission Control must use first-class mission records and typed links, not infer missions from issue labels or parent trees except through documented compatibility behavior.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"A UI built before projection semantics stabilize could duplicate state logic or obscure stale projection and validation problems.\"],\"validation\":[\"Linked issues prove JSON projection schema, mission progress, blockers, evidence gaps, workflow health, TUI fixture coverage, and Milestone 6 closeout validation.\"],\"work\":[]}"
links:
- target_id: "atelier-0003"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-0006"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-000c"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-001o"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-001x"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-001y"
  target_kind: "issue"
  type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "open"
title: "Mission Control projection and UI"
updated_at: "2026-06-11T00:06:58.518089056+00:00"
---

Expose durable Atelier state through deterministic Mission Control projections and an optional read-only terminal UI that consumes those projections instead of inventing a separate state model.
