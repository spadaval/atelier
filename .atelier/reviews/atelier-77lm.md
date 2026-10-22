---
schema: atelier.review
schema_version: 1
id: atelier-77lm
title: 'low: crates/atelier-cli/src/commands/git.rs:1011 (readiness) / crates/atelier-app/src/review_gate.rs approval_state: the mark reads review.current only, so a branch with an independent approval but an open FIX NOW finding shows ''✓ approved'' while merge --check fails review.findings. Matches the Done-when (approval state only) but an orchestrator may read it as ready to land. Consider documenting that the mark is review.current alone, or adding an ''open findings'' fact later.'
target: atelier-tciw
identity: Independent reviewer (Claude subagent)
commit: 14889fd20a35ed0c570dee38564353a9d26b9127
created_at: 2026-10-22T22:29:50.670812730Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/git.rs:1011 (readiness) / crates/atelier-app/src/review_gate.rs approval_state: the mark reads review.current only, so a branch with an independent approval but an open FIX NOW finding shows '✓ approved' while merge --check fails review.findings. Matches the Done-when (approval state only) but an orchestrator may read it as ready to land. Consider documenting that the mark is review.current alone, or adding an 'open findings' fact later.
