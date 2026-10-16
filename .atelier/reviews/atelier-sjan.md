---
schema: atelier.review
schema_version: 1
id: atelier-sjan
title: 'docs/adr/0022-status-orients-and-output-is-dense.md:10-12 understates ownerless prime: it also shows checkout clean/dirty, policy diagnostics, up to 8 Active rows, a ''Needs a human: N'' count when N>0 (so ''no counts'' is not quite true), and a second static line, ''Before handing off: record findings and honest proof, commit records with the code, then run atelier land'' (session.rs:209), plus ''Select ready work with atelier work ready''. Decision 2 (''commands print state and next actions, not instructions'') does not say whether the handoff line survives; atelier-sdb0 keeps it (removes only the Guidance line). Also Decision 1 says status shows ''ready and needs-a-human work'' but status shows only counts and no follow-up adds lists. Fix: describe prime accurately and say explicitly whether the handoff line is kept, so sdb0''s implementer need not decide the ADR''s question.'
target: atelier-bw0b
identity: Independent reviewer (Claude subagent)
commit: 484e6d6fd514738417d0297e2bc5dd376b181e86
created_at: 2026-10-16T23:21:54.197245049Z
event:
  event: finding
  recommendation: fix_now
---

docs/adr/0022-status-orients-and-output-is-dense.md:10-12 understates ownerless prime: it also shows checkout clean/dirty, policy diagnostics, up to 8 Active rows, a 'Needs a human: N' count when N>0 (so 'no counts' is not quite true), and a second static line, 'Before handing off: record findings and honest proof, commit records with the code, then run atelier land' (session.rs:209), plus 'Select ready work with atelier work ready'. Decision 2 ('commands print state and next actions, not instructions') does not say whether the handoff line survives; atelier-sdb0 keeps it (removes only the Guidance line). Also Decision 1 says status shows 'ready and needs-a-human work' but status shows only counts and no follow-up adds lists. Fix: describe prime accurately and say explicitly whether the handoff line is kept, so sdb0's implementer need not decide the ADR's question.
