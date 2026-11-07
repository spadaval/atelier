---
schema: atelier.review
schema_version: 2
id: atelier-r22g
title: 'crates/atelier-app/src/criteria.rs:101, criteria.rs:218-223, lint.rs:268-271: residue from the criteria_mission fix. criteria::criteria_mission (the read-time owner lookup that jye5 was about) has no callers left; MissionProof.orphaned is documented as ''Receipts in the mission''s scope'' and criteria_findings as checking ''the target''s mission'' and scoping by target, but both now key on the stored criteria_mission. A later reader could reintroduce read-time attribution by reusing the dead helper. Fix: delete criteria_mission(), and reword the two doc comments to say the stored mission decides.'
target: atelier-ho7o
identity: Independent reviewer 3 (Claude subagent)
commit: 8f3e78189c614d14110a199469ca8441cc7f7293
created_at: 2026-10-17T18:58:42.212937117Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 8f3e78189c614d14110a199469ca8441cc7f7293
    recorded_at: 2026-10-17T19:04:45.069571867Z
    follow_up: atelier-pth2
    reason: 'Accept the reviewer''s DEFER: cleanup only; follow-up atelier-pth2'
    migrated_from: atelier-ct6h
---

crates/atelier-app/src/criteria.rs:101, criteria.rs:218-223, lint.rs:268-271: residue from the criteria_mission fix. criteria::criteria_mission (the read-time owner lookup that jye5 was about) has no callers left; MissionProof.orphaned is documented as 'Receipts in the mission's scope' and criteria_findings as checking 'the target's mission' and scoping by target, but both now key on the stored criteria_mission. A later reader could reintroduce read-time attribution by reusing the dead helper. Fix: delete criteria_mission(), and reword the two doc comments to say the stored mission decides.
