---
schema: atelier.review
schema_version: 1
id: atelier-fvyl
title: 'Inherited blockers are reported as direct and do not appear in issue show. With P blocked_by B and K --parent P: issue show K prints ''Blocked by (none)'' (crates/atelier-cli/src/commands/issue.rs:126 still uses index.get_blockers), while its Transition Readiness says ''start: blocked - task K has direct blocker path K -> B'' (mission_readiness dependency_diagnosis, fed by the closure path, which leaves out the parent hop). The same wrong ''direct blocker path'' text appears in issue transition K and atelier start K. An agent following the ''Next: atelier issue show B'' hint is told that K directly depends on B, but K''s own record has no such link, and the Blocked by section contradicts the refusal. The record asks for output that names the parent''s blocker; it is named, but described wrongly. Fix: show inherited blockers in issue show (e.g. ''Blocked by (via parent P)''), and make the diagnosis path or wording say the blocker is inherited from parent P, e.g. path K -> P -> B or ''blocker B inherited from parent P''.'
target: atelier-e4c5
identity: Independent reviewer (Claude subagent)
commit: 7c7cb69cb8d1c0df8bf8d41c6bc6217a6d671ef0
created_at: 2026-10-16T00:51:31.972810674Z
event:
  event: finding
  recommendation: fix_now
---

Inherited blockers are reported as direct and do not appear in issue show. With P blocked_by B and K --parent P: issue show K prints 'Blocked by (none)' (crates/atelier-cli/src/commands/issue.rs:126 still uses index.get_blockers), while its Transition Readiness says 'start: blocked - task K has direct blocker path K -> B' (mission_readiness dependency_diagnosis, fed by the closure path, which leaves out the parent hop). The same wrong 'direct blocker path' text appears in issue transition K and atelier start K. An agent following the 'Next: atelier issue show B' hint is told that K directly depends on B, but K's own record has no such link, and the Blocked by section contradicts the refusal. The record asks for output that names the parent's blocker; it is named, but described wrongly. Fix: show inherited blockers in issue show (e.g. 'Blocked by (via parent P)'), and make the diagnosis path or wording say the blocker is inherited from parent P, e.g. path K -> P -> B or 'blocker B inherited from parent P'.
