---
schema: atelier.review
schema_version: 1
id: atelier-hknd
title: finding on atelier-hhce @0c3835b9
target: atelier-hhce
identity: claude-reviewer-hhce
commit: 0c3835b9ff81af67b5fbe9c2015749f453f1b0a4
created_at: 2026-11-04T23:36:50.384742052Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/evidence.rs:253 evidence record --criterion S1=pass on a mission with no criteria is refused correctly, but the message says 'Name the criteria as the mission's Success Criteria bullets spell them (atelier work mission <id> lists them)'. work mission lists none, so the hint sends the agent looking for bullets that don't exist. Fix: when the mission defines no criteria, say 'mission <id> defines no success criteria; drop --criterion'.
