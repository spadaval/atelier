---
schema: atelier.review
schema_version: 2
id: atelier-2boy
title: 'crates/atelier-cli/src/commands/mission_review.rs:607-608: the doc comment on rich() still says a review or receipt is shown ''by its ID'', which is now wrong after b338068c. A reader would expect the old behaviour. Fix: say ''a review or receipt by the start of its summary''.'
target: atelier-nexd
identity: Independent reviewer 3 (Claude subagent)
commit: 1f8465d99fbeaa96241d5b0f1031bf6259a43aa9
created_at: 2026-10-17T22:12:51.259586078Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 1f8465d99fbeaa96241d5b0f1031bf6259a43aa9
    recorded_at: 2026-10-17T22:13:24.199506588Z
    follow_up: atelier-lyow
    reason: Accept the reviewer's DEFER; follow-up atelier-lyow
    migrated_from: atelier-ip5r
---

crates/atelier-cli/src/commands/mission_review.rs:607-608: the doc comment on rich() still says a review or receipt is shown 'by its ID', which is now wrong after b338068c. A reader would expect the old behaviour. Fix: say 'a review or receipt by the start of its summary'.
