---
schema: atelier.review
schema_version: 1
id: atelier-z7pj
title: 'crates/atelier-cli/src/commands/mission_review.rs:1029-1047: the brief''s HTML disagrees with its Markdown on mission review events. (a) Each HTML event line drops the event ID, which the Markdown (line ~810) and the debrief''s HTML (line ~631) both show, so a human reading the page cannot name the approval or finding to an agent. (b) When the mission has review events but none is an approval (for example a finding on the mission), the Markdown still prints ''No plan approval recorded.'' (line 820) but the HTML prints only the events; only the tile count says 0. Reproduced in a scratch repo with a finding on the mission. brief.rs''s parity loop cannot catch either, because it checks only summaries and IDs that appear elsewhere on the page (the same gap would hide a dropped Blocked-by cell, since the blocker''s ID is also a scope row). Fix: add escape(&event.id) to the HTML line, print the no-approval line whenever approvals == 0 in both forms, and have the parity test assert the approval event''s ID and the blocked-by cell in the HTML.'
target: atelier-acrt
identity: Independent reviewer (Claude subagent)
commit: 5c858f437d2b7227bde75d9eaa3c650240b510b7
created_at: 2026-10-17T19:52:24.744885494Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/mission_review.rs:1029-1047: the brief's HTML disagrees with its Markdown on mission review events. (a) Each HTML event line drops the event ID, which the Markdown (line ~810) and the debrief's HTML (line ~631) both show, so a human reading the page cannot name the approval or finding to an agent. (b) When the mission has review events but none is an approval (for example a finding on the mission), the Markdown still prints 'No plan approval recorded.' (line 820) but the HTML prints only the events; only the tile count says 0. Reproduced in a scratch repo with a finding on the mission. brief.rs's parity loop cannot catch either, because it checks only summaries and IDs that appear elsewhere on the page (the same gap would hide a dropped Blocked-by cell, since the blocker's ID is also a scope row). Fix: add escape(&event.id) to the HTML line, print the no-approval line whenever approvals == 0 in both forms, and have the parity test assert the approval event's ID and the blocked-by cell in the HTML.
