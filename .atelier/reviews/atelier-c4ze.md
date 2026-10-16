---
schema: atelier.review
schema_version: 1
id: atelier-c4ze
title: 'Item 3''s ''no longer reproduces'' claim in .atelier/issues/atelier-v44v.md (Design, item 3) only holds for the whole-repo check. ''atelier issue create X --issue-type mission'' succeeds and prints ''Validate this issue: atelier check <id>''. Running that suggested command fails with ''Issue section Outcome must be present and non-empty'' and ''Issue section Success criteria must be present and non-empty'' (rc=1). Plain ''atelier check'' passes. The strategy-link half no longer reproduces, but the create/check mismatch for success criteria does. A bare task behaves the same way (missing Goal and Done when), so this looks like intended draft-readiness linting. Record it as accepted with that reason instead of saying it no longer reproduces.'
target: atelier-v44v
identity: Independent reviewer (Claude subagent)
commit: 656cef00ae542b1bad641b1c8392e5ad37dd5069
created_at: 2026-10-16T19:30:52.061639806Z
event:
  event: finding
  recommendation: fix_now
---

Item 3's 'no longer reproduces' claim in .atelier/issues/atelier-v44v.md (Design, item 3) only holds for the whole-repo check. 'atelier issue create X --issue-type mission' succeeds and prints 'Validate this issue: atelier check <id>'. Running that suggested command fails with 'Issue section Outcome must be present and non-empty' and 'Issue section Success criteria must be present and non-empty' (rc=1). Plain 'atelier check' passes. The strategy-link half no longer reproduces, but the create/check mismatch for success criteria does. A bare task behaves the same way (missing Goal and Done when), so this looks like intended draft-readiness linting. Record it as accepted with that reason instead of saying it no longer reproduces.
