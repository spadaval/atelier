---
schema: atelier.review
schema_version: 1
id: atelier-j8y1
title: 'skills/agent-factory/references/tracker-records.md:39-41 (Mission; vendored copy identical): ''If there is no separate strategy file, the mission body is the strategy: state the success criteria here and add other strategy template sections only as needed.'' Atelier''s mission body accepts only Outcome, Success criteria, Epics (+Notes, Close Reason); parse_issue_sections_for_type in crates/atelier-records/src/lib.rs rejects anything else. Reproduced with ''atelier issue create m --issue-type mission --body ...'': ''## Why'' -> ''Unknown issue body section Why''; ''## Scope'' -> ''section Scope is not valid for mission''. Shape, Authority, and Risks and Unknowns are rejected the same way. An agent following the skill for a mission with no strategy file gets a create failure, or if it hand-edits the record, makes atelier check fail for the whole repo. Fix: say that without a strategy file the mission body carries only Outcome and Success criteria, and anything else (scope, authority, risks) goes in a strategy file linked via the strategy field.'
target: atelier-kr30
identity: Independent reviewer (Claude subagent)
commit: 9b61daecaa3d48e3f1657d488d81fb32a1c0fc3a
created_at: 2026-10-15T19:58:25.519463406Z
event:
  event: finding
  recommendation: fix_now
---

skills/agent-factory/references/tracker-records.md:39-41 (Mission; vendored copy identical): 'If there is no separate strategy file, the mission body is the strategy: state the success criteria here and add other strategy template sections only as needed.' Atelier's mission body accepts only Outcome, Success criteria, Epics (+Notes, Close Reason); parse_issue_sections_for_type in crates/atelier-records/src/lib.rs rejects anything else. Reproduced with 'atelier issue create m --issue-type mission --body ...': '## Why' -> 'Unknown issue body section Why'; '## Scope' -> 'section Scope is not valid for mission'. Shape, Authority, and Risks and Unknowns are rejected the same way. An agent following the skill for a mission with no strategy file gets a create failure, or if it hand-edits the record, makes atelier check fail for the whole repo. Fix: say that without a strategy file the mission body carries only Outcome and Success criteria, and anything else (scope, authority, risks) goes in a strategy file linked via the strategy field.
