---
schema: atelier.review
schema_version: 2
id: atelier-vw77
title: finding on atelier-qr5s @e955ba3e
target: atelier-qr5s
identity: gpt-6-astra-plan-r1
commit: e955ba3ee79817bdaf94af5edef3c7139ddbf720
created_at: 2026-11-10T01:00:02.825934644Z
event:
  event: finding
  recommendation: fix_now
---

high atelier-wd1i and docs/strategies/redesign-3-cutover.md:104-106: cutover requires no active work or running thread in any repository, but its own old-schema item and orchestrator must be active to convert and install. The conversion cannot be merged normally after deleting the tracking record, and pinning CI to 0.4.0 then rejects converted records. Specify a reviewed preparation item merged before a narrow maintenance phase, exempt only the maintenance thread, and define conversion publication, CI-version switch, completion tracking and rollback as one rehearsed protocol.
