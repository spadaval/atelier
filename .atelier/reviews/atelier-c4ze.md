---
schema: atelier.review
schema_version: 2
id: atelier-c4ze
title: 'Item 3''s ''no longer reproduces'' claim in .atelier/issues/atelier-v44v.md (Design, item 3) only holds for the whole-repo check. ''atelier issue create X --issue-type mission'' succeeds and prints ''Validate this issue: atelier check <id>''. Running that suggested command fails with ''Issue section Outcome must be present and non-empty'' and ''Issue section Success criteria must be present and non-empty'' (rc=1). Plain ''atelier check'' passes. The strategy-link half no longer reproduces, but the create/check mismatch for success criteria does. A bare task behaves the same way (missing Goal and Done when), so this looks like intended draft-readiness linting. Record it as accepted with that reason instead of saying it no longer reproduces.'
target: atelier-v44v
identity: Independent reviewer (Claude subagent)
commit: 656cef00ae542b1bad641b1c8392e5ad37dd5069
created_at: 2026-10-16T19:30:52.061639806Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 40a0de63cd3897f24acbfcf963d1311137a6cf33
    recorded_at: 2026-10-16T19:37:48.050437640Z
    resolution_commit: 496a92fb61d95e525061beedfa54fd43d187fdea
    reason: 'Resolved. Design item 3 now says check <id> still flags a bare mission and records that as intended. Reproduced on the branch binary in a fresh repo: whole-repo ''atelier check'' passes after creating a bare mission and a bare task; ''check <id>'' reports Outcome/Success criteria for the mission and Goal/Done when for the task; ''issue transition'' shows ready blocked by issue.sections_parseable. The text is accurate.'
    migrated_from: atelier-knb3
---

Item 3's 'no longer reproduces' claim in .atelier/issues/atelier-v44v.md (Design, item 3) only holds for the whole-repo check. 'atelier issue create X --issue-type mission' succeeds and prints 'Validate this issue: atelier check <id>'. Running that suggested command fails with 'Issue section Outcome must be present and non-empty' and 'Issue section Success criteria must be present and non-empty' (rc=1). Plain 'atelier check' passes. The strategy-link half no longer reproduces, but the create/check mismatch for success criteria does. A bare task behaves the same way (missing Goal and Done when), so this looks like intended draft-readiness linting. Record it as accepted with that reason instead of saying it no longer reproduces.
