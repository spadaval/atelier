---
schema: atelier.review
schema_version: 2
id: atelier-kikm
title: 'mission_overview.rs:131,153: open work in a done mission''s scope is neither unassigned nor grouped, only counted (Linked only to done missions: N); the mission close transition allows open advances work, so the finished mission''s leftover work is unnamed in status and work missions between missions.'
target: atelier-ihns
identity: Independent reviewer (Claude subagent)
commit: c5b67d0b4c4a83150e2fa341f88c10062fbb9013
created_at: 2026-10-17T03:20:06.663087060Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: 754415a06d4b4149d9528562b8e1900fd6465678
    recorded_at: 2026-10-17T03:25:40.027575865Z
    resolution_commit: 11b62c89980bd35a66e8565401b0ba270f3d5b82
    reason: Unassigned now means outside every open mission's scope, so a done mission's leftover work is grouped and named; test closes a mission with open scope.
    migrated_from: atelier-okl1
---

mission_overview.rs:131,153: open work in a done mission's scope is neither unassigned nor grouped, only counted (Linked only to done missions: N); the mission close transition allows open advances work, so the finished mission's leftover work is unnamed in status and work missions between missions.
