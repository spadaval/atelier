---
schema: atelier.review
schema_version: 2
id: atelier-77lm
title: 'low: crates/atelier-cli/src/commands/git.rs:1011 (readiness) / crates/atelier-app/src/review_gate.rs approval_state: the mark reads review.current only, so a branch with an independent approval but an open FIX NOW finding shows ''✓ approved'' while merge --check fails review.findings. Matches the Done-when (approval state only) but an orchestrator may read it as ready to land. Consider documenting that the mark is review.current alone, or adding an ''open findings'' fact later.'
target: atelier-tciw
identity: Independent reviewer (Claude subagent)
commit: 14889fd20a35ed0c570dee38564353a9d26b9127
created_at: 2026-10-22T22:29:50.670812730Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 20e6fe4e81af429976db7a8f3ed4e041e6d46d50
    recorded_at: 2026-10-22T22:30:14.248961108Z
    follow_up: atelier-efnf
    reason: Tracked in atelier-efnf.
    migrated_from: atelier-yyxo
---

low: crates/atelier-cli/src/commands/git.rs:1011 (readiness) / crates/atelier-app/src/review_gate.rs approval_state: the mark reads review.current only, so a branch with an independent approval but an open FIX NOW finding shows '✓ approved' while merge --check fails review.findings. Matches the Done-when (approval state only) but an orchestrator may read it as ready to land. Consider documenting that the mark is review.current alone, or adding an 'open findings' fact later.
