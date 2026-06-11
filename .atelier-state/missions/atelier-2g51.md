---
created_at: "2026-06-11T22:14:53.209649393+00:00"
id: "atelier-2g51"
data: "{\"constraints\":[\"Mission closeout requires all linked work closed, evidence attached, workflow validators passing, and a clean Git worktree.\",\"Keep mission close out of v1; status=closed is the single enforced transition path.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[],\"validation\":[\"workflow validate fails nonzero on validator failure\",\"mission update --status closed rejects dirty worktree and other closeout blockers\",\"mission status reports concrete closeout blockers\",\"final closeout records cargo fmt, cargo nextest, export --check, lint, doctor, and clean git status\"],\"work\":[]}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1e24"
    role: "advances"
  - kind: "issue"
    id: "atelier-3gki"
    role: "advances"
  - kind: "issue"
    id: "atelier-481n"
    role: "advances"
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Make missions actually work"
updated_at: "2026-06-11T22:24:19.580403169+00:00"
---

Turn missions from advisory records into enforced workflow objects. Validators must return enforceable results, failed validators must fail commands, and mission closeout must be impossible while required gates fail, including a dirty Git worktree.
